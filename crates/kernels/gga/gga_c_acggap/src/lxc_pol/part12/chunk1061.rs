//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1061/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1061<F: Float>(t36006: F, t36010: F, t36017: F, t36030: F, t36032: F, t36036: F, t36039: F, t36041: F, t31689: F, t36004: F, t36014: F, t36022: F, t36026: F, t36044: F, t36047: F, t36050: F, t36053: F, t36056: F) -> (F,) {
    let t37826 = 0.34299214494455789578e-2 * t36006;
    let t37827 = 0.20965394859736101379e-2 * t36010;
    let t37830 = 0.68598428988911579156e-2 * t36017;
    let t37833 = 0.62896184579208304138e-3 * t36030;
    let t37834 = 0.1324375e0 * t36032;
    let t37835 = 0.1528125e-1 * t36036;
    let t37836 = 7.0 / 12.0 * t36039;
    let t37837 = 7.0 / 36.0 * t36041;
    let t37843 = -0.20965394859736101378e-2 * t36004 - t37826 - t37827 + 0.12579236915841660828e-2 * t36014 + 0.21437009059034868486e-2 * t31689 - t37830 + 0.62896184579208304138e-3 * t36022 - 0.31448092289604152068e-2 * t36026 - t37833 + t37834 + t37835 - t37836 - t37837 + t36044 / 4.0 + t36047 / 4.0 + t36050 / 8.0 + t36053 / 12.0 + t36056 / 12.0;
    (t37843,)
}
