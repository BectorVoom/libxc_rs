//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1243/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1243(t31224: f64, t32739: f64, t32740: f64, t35418: f64, t35424: f64, t37527: f64, t37528: f64, t37529: f64, t37531: f64, t37533: f64, t37534: f64, t39937: f64, t39939: f64, t39944: f64, t39946: f64, t39948: f64, t39950: f64, t39952: f64) -> f64 {
    let t41866 = -0.68598428988911579156e-2_f64 * t39937 - 0.94344276868812456204e-2_f64 * t39939 + t37527 - t37528 + t37529 + t37531 - 0.62896184579208304136e-2_f64 * t39944 - 0.38586616306262763275e-1_f64 * t39946 + 0.80031500487063509015e-2_f64 * t39948 - t37533 - t37534 + 0.68598428988911579156e-2_f64 * t39950 + 0.12862205435420921092e-2_f64 * t39952 - 0.45017719023973223822e-1_f64 * t31224 + t32739 + 0.13208198761633743869e0_f64 * t35418 + t32740 + t35424;
    t41866
}
