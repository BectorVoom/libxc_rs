//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1385/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1385(t93826: f64, t1295: f64, t15793: f64, t2169: f64, t233: f64, t235: f64, t27743: f64, t27755: f64, t27758: f64, t28300: f64, t4533: f64, t7673: f64, t911: f64, t915: f64, t92157: f64, t92165: f64, t92168: f64, t92170: f64, t92339: f64, t93817: f64) -> f64 {
    let t97584 = 2.0_f64 * t93826;
    let t97585 = t92157 + t7673 * t27758 / 8.0_f64 - t233 * t915 * t28300 / 8.0_f64 + t911 * t27743 / 8.0_f64 + t93817 + t911 * t27755 / 8.0_f64 - t92165 - t2169 * t235 * t15793 / 16.0_f64 - t2169 * t4533 * t1295 / 8.0_f64 + t97584 + t92168 + t92170 + t92339;
    t97585
}
