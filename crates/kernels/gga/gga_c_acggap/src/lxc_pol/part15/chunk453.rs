//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 453/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk453<F: Float>(t2176: F, t323: F, t1968: F, t1970: F, t1986: F, t1989: F, t1995: F, t1999: F, t2010: F, t2013: F, t2017: F, t2021: F, t2023: F, t2038: F, t1974: F, t2002: F, t2004: F, t2006: F, t2026: F, t2033: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2178 = 0.65854491829355115987e0 * t2176 * t323;
    let t2179 = 0.18868855373762491241e-2 * t1968;
    let t2180 = 0.12862205435420921092e-2 * t1970;
    let t2182 = 0.14291339372689912324e-3 * t1986;
    let t2183 = 0.31448092289604152069e-3 * t1989;
    let t2184 = 0.20965394859736101379e-3 * t1995;
    let t2185 = 0.85748036236139473944e-3 * t1999;
    let t2189 = 0.40015750243531754507e-2 * t2010;
    let t2190 = 0.85748036236139473944e-3 * t2013;
    let t2191 = 0.28015625e-1 * t2017;
    let t2192 = 7.0 / 144.0 * t2021;
    let t2193 = 11.0 / 576.0 * t2023;
    let t2196 = t2038 / 96.0;
    let t2197 = t2179 - t2180 + 0.21437009059034868486e-3 * t1974 + t2182 - t2183 - t2184 - t2185 - 0.34299214494455789578e-2 * t2002 + 0.17149607247227894789e-2 * t2004 - 0.17149607247227894789e-2 * t2006 - t2189 + t2190 + t2191 - t2192 - t2193 + t2026 / 48.0 + 0.22921875e-1 * t2033 + t2196;
    (t2178, t2179, t2180, t2182, t2183, t2184, t2185, t2189, t2190, t2191, t2192, t2193, t2196, t2197)
}
