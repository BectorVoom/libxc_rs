//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1234/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1234(t32621: f64, t32622: f64, t32627: f64, t32628: f64, t35034: f64, t35043: f64, t37361: f64, t37362: f64, t37365: f64, t39653: f64, t39658: f64, t39661: f64, t39665: f64, t39669: f64, t39673: f64, t39675: f64, t39679: f64, t39683: f64) -> f64 {
    let t41748 = 0.13719685797782315831e-1_f64 * t39653 - t32621 - t32622 + t35034 - t32627 - t32628 + 0.1528125e-1_f64 * t39658 + t39661 / 12.0_f64 + t37361 + t37362 - 0.64311027177104605458e-2_f64 * t39665 + 0.10718504529517434243e-2_f64 * t39669 - 35.0_f64 / 54.0_f64 * t35043 - 0.14291339372689912324e-3_f64 * t39673 - t37365 + 0.31448092289604152069e-3_f64 * t39675 + 0.31448092289604152069e-3_f64 * t39679 + 0.18868855373762491241e-2_f64 * t39683;
    t41748
}
