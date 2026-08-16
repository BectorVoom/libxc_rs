//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 965/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk965<F: Float>(t2030: F, t987: F, t475: F, t125: F, t2037: F, t296: F, t5625: F, t5629: F, t5653: F, t5661: F, t5674: F, t5678: F, t5690: F, t5694: F, t5703: F, t5707: F, t5710: F, t5713: F, t5717: F, t5733: F, t6028: F, t6032: F, t8331: F, t8332: F, t8335: F, t8341: F, t8347: F, t8351: F, t8355: F, t8357: F, t8379: F, t8473: F, t8493: F, t968: F, t988: F) -> F {
    let t8496 = t987 * t2030;
    let t8497 = t475 * t8496;
    let t8500 = F::cast_from(0.39633663517353708522e0_f64) * t5674 - F::cast_from(0.29056741517886919367e-3_f64) * t5678 + F::cast_from(2.0_f64) * t988 * t5625 - F::cast_from(0.11622696607154767747e-2_f64) * t5690 - t5694 + F::cast_from(12.0_f64) * t8331 * t8332 + F::cast_from(6.0_f64) * t8335 * t968 - F::cast_from(2.0_f64) * t988 * t5661 - t988 * t5629 + F::cast_from(6.0_f64) * t8341 * t2037 + (-F::cast_from(0.28298369577492776242e0_f64) * t5703 - t5707 + F::cast_from(0.53059442957798955452e-1_f64) * t5710 + F::cast_from(0.2122377718311958218e0_f64) * t5713 + t5717 + F::cast_from(0.3199504064530762818e0_f64) * t8347 + F::cast_from(0.6399008129061525636e0_f64) * t5733 - t8351 + t8355 + F::cast_from(0.1061188859155979109e0_f64) * t8357 + t8379) * t296 + (t8473 + t8493) * t125 - F::cast_from(6.0_f64) * t8497 * t5653 + t6028 - t6032;
    t8500
}
