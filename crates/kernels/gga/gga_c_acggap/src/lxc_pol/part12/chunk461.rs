//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 461/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk461<F: Float>(t2023: F, t2038: F, t1974: F, t2002: F, t2004: F, t2006: F, t2026: F, t2033: F, t2179: F, t2180: F, t2182: F, t2183: F, t2184: F, t2185: F, t2189: F, t2190: F, t2191: F, t2192: F) -> (F, F, F) {
    let t2193 = F::new(11.0) / F::new(576.0) * t2023;
    let t2196 = t2038 / F::new(96.0);
    let t2197 = t2179 - t2180 + F::new(0.21437009059034868486e-3) * t1974 + t2182 - t2183 - t2184 - t2185 - F::new(0.34299214494455789578e-2) * t2002 + F::new(0.17149607247227894789e-2) * t2004 - F::new(0.17149607247227894789e-2) * t2006 - t2189 + t2190 + t2191 - t2192 - t2193 + t2026 / F::new(48.0) + F::new(0.22921875e-1) * t2033 + t2196;
    (t2193, t2196, t2197)
}
