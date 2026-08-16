//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1275/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1275(t8235: f64, t832: f64, t853: f64, t2235: f64, t8214: f64, t2328: f64, t8300: f64, t2298: f64, t8012: f64, t898: f64, t2317: f64, t3161: f64, t8098: f64) -> (f64, f64, f64, f64, f64) {
    let t22357 = t8235 * t832;
    let t22359 = 3.0_f64 * t22357 * t853;
    let t22361 = 3.0_f64 * t8214 * t2235;
    let t22363 = 0.17544670867903938621e1_f64 * t2328 * t8300;
    let t22366 = 0.10526802520742363173e2_f64 * t898 * t8012 * t2298;
    let t22374 = 0.51947577317044391277e2_f64 * t898 * t2317 * t8098 * t3161;
    (t22359, t22361, t22363, t22366, t22374)
}
