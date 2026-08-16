//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1833/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1833(t25: f64, t19593: f64, t1408: f64, t6305: f64, t12061: f64, t20216: f64, t5134: f64, t514: f64, t5397: f64, t1649: f64, t6312: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t20372 = 12.0_f64 * t19593;
    let t20376 = t6305 * t1408;
    let t20384 = piecewise3(t26, 0.0_f64, -8.0_f64 / 27.0_f64 * t12061 * t20376 + 4.0_f64 / 3.0_f64 * t5134 * t5397 + 4.0_f64 / 3.0_f64 * t514 * t20216);
    let t20385 = t6312 * t1649;
    let t20390 = -t20216;
    (t20372, t20376, t20384, t20385, t20390)
}
