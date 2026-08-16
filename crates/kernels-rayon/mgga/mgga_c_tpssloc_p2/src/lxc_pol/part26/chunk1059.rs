//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1059/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1059(t12512: f64, t3: f64, t112: f64, t3931: f64, t111: f64, t1395: f64, t2319: f64, t671: f64, t2363: f64, t1401: f64, t3938: f64, t3941: f64, t576: f64, t577: f64, t9416: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12513 = t3 * t12512;
    let t12521 = t3931 * t112;
    let t12524 = t1395 * t111;
    let t12529 = t2319 * t671;
    let t12532 = t671 * t2363;
    let t12537 = 0.45e1_f64 * t12512 * t577 + 0.405e2_f64 * t12521 * t671 + 81.0_f64 * t12524 * t2319 + 0.405e2_f64 * t3938 * t2363 + 27.0_f64 * t576 * t12529 + 81.0_f64 * t3941 * t12532 + 0.135e2_f64 * t1401 * t9416;
    (t12513, t12521, t12524, t12529, t12532, t12537)
}
