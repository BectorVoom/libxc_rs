//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1939;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1940;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta603(t18531: f64, t25245: f64, t18432: f64, t93025: f64, t18440: f64, t25227: f64, t2661: f64, t18437: f64, t7045: f64, t18348: f64, t1945: f64, t807: f64, t25266: f64, t6019: f64, t6024: f64, t93054: f64, t18495: f64, t18500: f64, t18618: f64, t7038: f64, t18466: f64, t25270: f64, t18622: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t106048, t106050, t106053, t106058, t106061) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1939(t18531, t25245, t18432, t93025, t18440, t25227, t2661, t18437, t7045, t18348, t1945, t807);
        let (t106063, t106065, t106068, t106070, t106072, t106074, t106080) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1940(t25266, t6019, t6024, t93054, t18495, t7045, t18500, t18618, t7038, t18466, t25270, t18622, t25245);
    (t106048, t106050, t106053, t106058, t106061, t106063, t106065, t106068, t106070, t106072, t106074, t106080)
}
