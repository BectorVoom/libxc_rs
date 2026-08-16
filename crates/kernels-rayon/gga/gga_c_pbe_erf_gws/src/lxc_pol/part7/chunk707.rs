//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 707/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk707(t103: f64, t2: f64, t39: f64, t5772: f64, t102: f64, t120: f64, t5645: f64, t506: f64, t497: f64, t542: f64, t496: f64, t10: f64, t127: f64, t5744: f64, t5749: f64, t5751: f64, t5753: f64, t5755: f64, t5759: f64, t5764: f64, t5768: f64, t5771: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5773 = t103 * t2;
    let t5776 = 0.19486833333333333333e1_f64 * t5772 * t5773 * t39;
    let t5779 = 0.2923025e1_f64 * t102 * t120 * t5645;
    let t5780 = t506 * t5645;
    let t5783 = t542 * t497;
    let t5784 = t496 * t5783;
    let t5786 = 9.0_f64 / 2.0_f64 * t496 * t10 * t5744 - t5749 - t5751 + t5753 - t5755 - t5759 - 0.146904e1_f64 * t5764 + 0.220356e1_f64 * t5768 + t5771 - t5776 - t5779 - 0.146904e1_f64 * t127 * t5780 - 2.0_f64 / 3.0_f64 * t5784;
    (t5773, t5776, t5779, t5780, t5783, t5786)
}
