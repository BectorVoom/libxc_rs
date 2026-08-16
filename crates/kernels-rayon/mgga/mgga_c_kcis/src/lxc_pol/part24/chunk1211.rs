//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1211/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1211(t1658: f64, t2169: f64, t233: f64, t28300: f64, t29223: f64, t29229: f64, t6883: f64, t7673: f64, t914: f64, t91791: f64, t91793: f64, t91863: f64, t91866: f64, t91869: f64, t91872: f64, t91874: f64) -> f64 {
    let t99810 = -t91791 - t91793 - t91863 + t7673 * t29229 / 8.0_f64 + t91866 - t91869 + t91872 - t91874 - t2169 * t914 * t6883 / 16.0_f64 - t233 * t1658 * t28300 / 8.0_f64 + t7673 * t29223 / 16.0_f64;
    t99810
}
