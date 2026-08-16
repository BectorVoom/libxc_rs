//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1212/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1212(t36006: f64, t36010: f64, t36017: f64, t36030: f64, t36032: f64, t36036: f64, t36039: f64, t36041: f64, t31689: f64, t36004: f64, t36014: f64, t36022: f64, t36026: f64, t36044: f64, t36047: f64, t36050: f64, t36053: f64, t36056: f64) -> f64 {
    let t37826 = 0.34299214494455789578e-2_f64 * t36006;
    let t37827 = 0.20965394859736101379e-2_f64 * t36010;
    let t37830 = 0.68598428988911579156e-2_f64 * t36017;
    let t37833 = 0.62896184579208304138e-3_f64 * t36030;
    let t37834 = 0.1324375e0_f64 * t36032;
    let t37835 = 0.1528125e-1_f64 * t36036;
    let t37836 = 7.0_f64 / 12.0_f64 * t36039;
    let t37837 = 7.0_f64 / 36.0_f64 * t36041;
    let t37843 = -0.20965394859736101378e-2_f64 * t36004 - t37826 - t37827 + 0.12579236915841660828e-2_f64 * t36014 + 0.21437009059034868486e-2_f64 * t31689 - t37830 + 0.62896184579208304138e-3_f64 * t36022 - 0.31448092289604152068e-2_f64 * t36026 - t37833 + t37834 + t37835 - t37836 - t37837 + t36044 / 4.0_f64 + t36047 / 4.0_f64 + t36050 / 8.0_f64 + t36053 / 12.0_f64 + t36056 / 12.0_f64;
    t37843
}
