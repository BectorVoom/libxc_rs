//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 932/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk932<F: Float>(t338: F, t1323: F, t164: F, t1309: F, t3966: F, t3984: F, t25: F, t3989: F, t13125: F, t1320: F, t1310: F, t122: F, t4000: F) -> (F, F, F, F, F) {
    let t400 = F::new(0.0) < t338;
    let t13804 = t164 * t1323;
    let t13805 = t1309 * t13804;
    let t13807 = t3966 * t3984;
    let t13809 = t25 * t3989;
    let t13810 = t1309 * t13809;
    let t13815 = piecewise3::<F>(t400, t13125, -t13125);
    let t13816 = t1320 * t13815;
    let t13817 = t1310 * t13816;
    let t13820 = t4000 * t122;
    (t13805, t13807, t13810, t13817, t13820)
}
