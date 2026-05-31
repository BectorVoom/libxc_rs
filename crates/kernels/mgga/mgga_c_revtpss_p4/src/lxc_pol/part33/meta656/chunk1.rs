//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2110/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2110<F: Float>(t114: F, t101454: F, t101456: F, t101754: F, t105870: F, t105873: F, t105876: F, t105878: F, t105881: F, t105883: F, t94974: F, t94976: F, t508: F, t651: F) -> (F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t105885 = -t94974 - F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t94976 - t101754 - t101454 + t101456 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t105870 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t105873 + t105876 / F::cast_from(2.0_f64) + t105878 / F::cast_from(3.0_f64) + t105881 / F::cast_from(4.0_f64) - t105883 / F::cast_from(8.0_f64);
    let t105886 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t105885);
    let t105889 = F::cast_from(2.0_f64) * t651 * t508 * t105886;
    (t105886, t105889)
}
