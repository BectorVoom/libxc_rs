//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 628/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk628(t4797: f64, t4799: f64, t4808: f64, t4816: f64, t3107: f64, t4802: f64, t4804: f64, t4812: f64, t4814: f64, t4820: f64, t4824: f64, t3110: f64, t3112: f64, t3118: f64, t3122: f64, t3128: f64, t3130: f64, t3142: f64, t3144: f64, t3146: f64, t3161: f64, t4834: f64) -> (f64, f64) {
    let t4856 = t4797 / 6.0_f64;
    let t4857 = 2.0_f64 / 3.0_f64 * t4799;
    let t4860 = t4808 / 12.0_f64;
    let t4863 = 4.0_f64 / 3.0_f64 * t4816;
    let t4865 = t4856 + t4857 - t4802 / 4.0_f64 + t4804 / 6.0_f64 - t4860 - t4812 / 12.0_f64 - 7.0_f64 / 9.0_f64 * t4814 - t4863 + t4820 + t4824 / 2.0_f64 + t3107;
    let t4874 = -t3110 + t3112 / 3.0_f64 + t3118 / 12.0_f64 - t3122 / 24.0_f64 - t3128 / 6.0_f64 - 2.0_f64 / 3.0_f64 * t3130 - t3142 - 14.0_f64 / 9.0_f64 * t3144 - 3.0_f64 / 2.0_f64 * t4834 + t3146 / 3.0_f64 + t3161;
    (t4865, t4874)
}
