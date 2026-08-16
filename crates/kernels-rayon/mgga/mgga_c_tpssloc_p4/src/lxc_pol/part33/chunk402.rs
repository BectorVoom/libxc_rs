//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 402/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk402(t1851: f64, t3: f64, t1401: f64, t1458: f64, t577: f64, t33: f64, t605: f64, t38: f64, t44: f64, t63: f64, t67: f64) -> (f64, f64, f64, f64, f64) {
    let t1852 = t3 * t1851;
    let t1858 = 0.45e1_f64 * t1851 * t577 + 0.135e2_f64 * t1401 * t1458;
    let t1860 = t605 * t33;
    let t1862 = t38 * t44 - t63;
    let t1863 = t1862 * t67;
    (t1852, t1858, t1860, t1862, t1863)
}
