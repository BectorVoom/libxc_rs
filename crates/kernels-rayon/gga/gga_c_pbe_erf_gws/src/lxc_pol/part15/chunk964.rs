//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 964/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk964(t2030: f64, t987: f64, t475: f64, t125: f64, t2037: f64, t296: f64, t5625: f64, t5629: f64, t5653: f64, t5661: f64, t5674: f64, t5678: f64, t5690: f64, t5694: f64, t5703: f64, t5707: f64, t5710: f64, t5713: f64, t5717: f64, t5733: f64, t6028: f64, t6032: f64, t8331: f64, t8332: f64, t8335: f64, t8341: f64, t8347: f64, t8351: f64, t8355: f64, t8357: f64, t8379: f64, t8473: f64, t8493: f64, t968: f64, t988: f64) -> f64 {
    let t8496 = t987 * t2030;
    let t8497 = t475 * t8496;
    let t8500 = 0.39633663517353708522e0_f64 * t5674 - 0.29056741517886919367e-3_f64 * t5678 + 2.0_f64 * t988 * t5625 - 0.11622696607154767747e-2_f64 * t5690 - t5694 + 12.0_f64 * t8331 * t8332 + 6.0_f64 * t8335 * t968 - 2.0_f64 * t988 * t5661 - t988 * t5629 + 6.0_f64 * t8341 * t2037 + (-0.28298369577492776242e0_f64 * t5703 - t5707 + 0.53059442957798955452e-1_f64 * t5710 + 0.2122377718311958218e0_f64 * t5713 + t5717 + 0.3199504064530762818e0_f64 * t8347 + 0.6399008129061525636e0_f64 * t5733 - t8351 + t8355 + 0.1061188859155979109e0_f64 * t8357 + t8379) * t296 + (t8473 + t8493) * t125 - 6.0_f64 * t8497 * t5653 + t6028 - t6032;
    t8500
}
