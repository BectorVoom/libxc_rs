//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1259/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1259(t125886: f64, t121076: f64, t121327: f64, t121337: f64, t122346: f64, t122498: f64, t122503: f64, t122504: f64, t125873: f64, t125875: f64, t125901: f64, t125903: f64, t27972: f64) -> f64 {
    let t128833 = 0.3718732920905101082e-4_f64 * t125886;
    let t128837 = 0.7437465841810202164e-3_f64 * t125873 + 0.14874931683620404328e-2_f64 * t125875 + t122498 + 0.3427184259906141157e1_f64 * t121076 * t122346 * t27972 + t128833 - 0.66934509195437693771e-4_f64 * t121327 + t121337 + 0.37645955677973955999e-4_f64 * t125901 - 0.66934509195437693771e-4_f64 * t125903 - t122503 + t122504;
    t128837
}
