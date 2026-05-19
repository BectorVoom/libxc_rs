//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1100/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1100<F: Float>(t874: F, t9486: F, t352: F, t2447: F, t4616: F, t876: F, t42023: F, t42026: F, t4905: F, t9540: F, t42044: F, t42057: F) -> (F, F, F, F, F, F, F) {
    let t43970 = t874 * t9486;
    let t43971 = t43970 * t352;
    let t43974 = t4616 * t2447;
    let t43975 = t43974 * t876;
    let t43978 = F::cast_from(0.162600798888400151e-2_f64) * t42023;
    let t43979 = F::cast_from(0.162600798888400151e-2_f64) * t42026;
    let t43981 = t9540 * t4905;
    let t43987 = F::cast_from(0.11918087970123395032e-3_f64) * t42044;
    let t43990 = F::cast_from(0.87811105813667929469e0_f64) * t42057;
    (t43971, t43975, t43978, t43979, t43981, t43987, t43990)
}
