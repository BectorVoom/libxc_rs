//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1009/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1009(t14849: f64, t14865: f64, t1591: f64, t4374: f64, t4497: f64, t6204: f64, t3969: f64, t4396: f64, t4369: f64, t1308: f64, t1056: f64, t4400: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t14866 = t14849 + t14865;
    let t14873 = t4374 * t1591;
    let t14874 = t14873 * t4497;
    let t14875 = t6204 * t14874;
    let t14878 = t4396 * t3969;
    let t14885 = t4369 * sigma0;
    let t14886 = t14885 * t1308;
    let t14891 = t1056 * t4497;
    let t14892 = t4400 * t14891;
    (t14866, t14875, t14878, t14886, t14892)
}
