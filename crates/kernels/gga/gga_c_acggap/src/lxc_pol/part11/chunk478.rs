//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 478/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk478<F: Float>(t2327: F, t598: F, t2022: F, t2024: F, t2039: F, t2078: F, t2084: F, t2098: F, t2102: F, t2291: F, t2295: F, t2300: F, t2305: F, t2307: F, t2311: F, t2315: F, t2319: F, t2321: F, t2323: F) -> (F,) {
    let t2328 = t598 * t2327;
    let t2330 = -t2022 - t2024 + 0.47172138434406228102e-3 * t2291 + t2039 - 0.21437009059034868486e-3 * t2295 + 0.7862023072401038017e-3 * t2300 - 0.31448092289604152068e-3 * t2305 - 0.42874018118069736972e-3 * t2307 - 0.53592522647587171215e-3 * t2311 + 0.114609375e-1 * t2315 + 0.7640625e-2 * t2319 + 0.85748036236139473944e-3 * t2321 - 0.85748036236139473944e-3 * t2323 + 0.10718504529517434243e-3 * t2328 + t2078 - t2084 - t2098 + t2102;
    (t2330,)
}
