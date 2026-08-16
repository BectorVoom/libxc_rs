//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1858/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1858(t1441: f64, t4072: f64, t19440: f64, t71: f64, t33: f64, t55880: f64, t5441: f64, t645: f64, t72: f64, t5389: f64, t641: f64, t12568: f64, t1410: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96356 = t1441 * t4072;
    let t96379 = t71 * t19440;
    let t96383 = t55880 * t33;
    let t96393 = t72 * t5441 * t645;
    let t96403 = t72 * t641 * t5389;
    let t96406 = t12568 * t1410;
    (t96356, t96379, t96383, t96393, t96403, t96406)
}
