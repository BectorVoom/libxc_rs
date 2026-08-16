//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1383/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1383(t2169: f64, t233: f64, t236: f64, t27155: f64, t27749: f64, t27752: f64, t2794: f64, t5398: f64, t7673: f64, t8021: f64, t8122: f64, t911: f64, t914: f64, t91791: f64, t91793: f64, t91863: f64, t91866: f64, t91869: f64, t91872: f64, t95511: f64, t97533: f64) -> f64 {
    let t97547 = -t91791 - t91793 - t91863 + t91866 - t2794 * t8122 / 8.0_f64 - t91869 - t233 * t236 * (t95511 + t97533) / 16.0_f64 - t2169 * t914 * t5398 / 8.0_f64 - t27155 * t8021 / 8.0_f64 + t7673 * t27749 / 8.0_f64 + t91872 + t911 * t27752 / 8.0_f64;
    t97547
}
