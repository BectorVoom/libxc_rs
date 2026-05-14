//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 472/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk472<F: Float>(t2192: F, t2193: F, t2196: F, t2206: F, t2207: F, t2210: F, t2211: F, t2291: F, t2295: F, t2300: F, t2305: F, t2307: F, t2311: F, t2315: F, t2319: F, t2321: F, t2323: F, t2328: F) -> (F,) {
    let t2384 = -t2192 - t2193 + 0.94344276868812456207e-3 * t2291 + t2196 - 0.42874018118069736972e-3 * t2295 + 0.15724046144802076034e-2 * t2300 - 0.62896184579208304138e-3 * t2305 - 0.85748036236139473944e-3 * t2307 - 0.10718504529517434243e-2 * t2311 + 0.22921875e-1 * t2315 + 0.1528125e-1 * t2319 + 0.17149607247227894789e-2 * t2321 - 0.17149607247227894789e-2 * t2323 + 0.21437009059034868486e-3 * t2328 + t2206 - t2207 - t2210 + t2211;
    (t2384,)
}
