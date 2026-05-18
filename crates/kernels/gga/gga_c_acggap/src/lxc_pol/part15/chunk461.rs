//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 461/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk461<F: Float>(t2077: F, t2083: F, t2097: F, t2101: F, t2042: F, t2044: F, t2049: F, t2053: F, t2057: F, t2063: F, t2071: F, t2075: F, t2088: F, t2093: F, t2105: F, t2110: F, t2114: F, t2119: F) -> (F, F, F, F, F) {
    let t2206 = F::new(0.21437009059034868486e-3) * t2077;
    let t2207 = F::new(0.21437009059034868486e-3) * t2083;
    let t2210 = F::new(0.1528125e-1) * t2097;
    let t2211 = F::new(0.31448092289604152069e-3) * t2101;
    let t2216 = -t2042 / F::new(24.0) - t2044 / F::new(48.0) - t2049 / F::new(64.0) - t2053 / F::new(192.0) - F::new(0.7640625e-2) * t2057 + F::new(0.1528125e-1) * t2063 - F::new(0.21437009059034868486e-3) * t2071 + F::new(0.31448092289604152069e-3) * t2075 + t2206 - t2207 - F::new(0.10718504529517434243e-2) * t2088 - F::new(0.42874018118069736972e-3) * t2093 - t2210 + t2211 + F::new(0.15724046144802076034e-2) * t2105 - F::new(0.62896184579208304138e-3) * t2110 + F::new(0.94344276868812456207e-3) * t2114 - F::new(0.85748036236139473944e-3) * t2119;
    (t2206, t2207, t2210, t2211, t2216)
}
