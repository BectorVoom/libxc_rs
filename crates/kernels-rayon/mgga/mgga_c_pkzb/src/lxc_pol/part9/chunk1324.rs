//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1324/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1324(t411: f64, t6546: f64, t3199: f64, t937: f64, t1245: f64, t6514: f64, t410: f64, t8309: f64, t1227: f64, t2421: f64, t2363: f64, t3246: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23398 = t411 * t6546;
    let t23412 = t937 * t3199;
    let t23416 = t6514 * t1245;
    let t23446 = t410 * t8309;
    let t23450 = t2421 * t1227;
    let t23465 = t2363 * t3246;
    (t23398, t23412, t23416, t23446, t23450, t23465)
}
