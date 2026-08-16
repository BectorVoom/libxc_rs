//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2521;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2522;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta615(t19666: f64, t4806: f64, t1042: f64, t16208: f64, t19661: f64, t1065: f64, t6258: f64, t906: f64, t5825: f64, t606: f64, t4801: f64, t1063: f64, t15668: f64, t15675: f64, t15707: f64, t19651: f64, t19659: f64, t19663: f64, t3127: f64, t3169: f64, t4837: f64, t4875: f64, t6302: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19667, t19668, t19671, t19672, t19675, t19676, t19677, t19680) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2521(t19666, t4806, t1042, t16208, t19661, t1065, t6258, t906, t5825, t606);
        let (t19681, t19682, t19685) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2522(t19680, t4801, t1042, t1063, t15668, t15675, t15707, t19651, t19659, t19663, t19668, t19672, t19677, t3127, t3169, t4837, t4875, t6302);
    (t19667, t19668, t19671, t19672, t19675, t19676, t19677, t19680, t19681, t19682, t19685)
}
