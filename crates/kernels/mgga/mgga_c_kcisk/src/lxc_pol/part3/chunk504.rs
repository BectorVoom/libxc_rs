//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 504/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk504<F: Float>(t338: F, t397: F, t3979: F, t403: F, t396: F, t1323: F, t25: F, t1309: F, t3729: F, t1320: F, t1310: F, t1293: F, t1300: F) -> (F, F, F, F, F, F, F, F) {
    let t400 = F::new(0.0) < t338;
    let t3981 = t397 * t3979 * t403;
    let t3983 = F::cast_from(0.11993859144118211475e-1_f64) * t396 * t3981;
    let t3984 = t25 * t1323;
    let t3985 = t1309 * t3984;
    let t3988 = piecewise3::<F>(t400, t3729, -t3729);
    let t3989 = t1320 * t3988;
    let t3990 = t1310 * t3989;
    let t3993 = t1293 * t1300;
    (t3981, t3983, t3984, t3985, t3988, t3989, t3990, t3993)
}
