//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 461/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk461(t2077: f64, t2083: f64, t2097: f64, t2101: f64, t2042: f64, t2044: f64, t2049: f64, t2053: f64, t2057: f64, t2063: f64, t2071: f64, t2075: f64, t2088: f64, t2093: f64, t2105: f64, t2110: f64, t2114: f64, t2119: f64) -> (f64, f64, f64, f64, f64) {
    let t2206 = 0.21437009059034868486e-3_f64 * t2077;
    let t2207 = 0.21437009059034868486e-3_f64 * t2083;
    let t2210 = 0.1528125e-1_f64 * t2097;
    let t2211 = 0.31448092289604152069e-3_f64 * t2101;
    let t2216 = -t2042 / 24.0_f64 - t2044 / 48.0_f64 - t2049 / 64.0_f64 - t2053 / 192.0_f64 - 0.7640625e-2_f64 * t2057 + 0.1528125e-1_f64 * t2063 - 0.21437009059034868486e-3_f64 * t2071 + 0.31448092289604152069e-3_f64 * t2075 + t2206 - t2207 - 0.10718504529517434243e-2_f64 * t2088 - 0.42874018118069736972e-3_f64 * t2093 - t2210 + t2211 + 0.15724046144802076034e-2_f64 * t2105 - 0.62896184579208304138e-3_f64 * t2110 + 0.94344276868812456207e-3_f64 * t2114 - 0.85748036236139473944e-3_f64 * t2119;
    (t2206, t2207, t2210, t2211, t2216)
}
