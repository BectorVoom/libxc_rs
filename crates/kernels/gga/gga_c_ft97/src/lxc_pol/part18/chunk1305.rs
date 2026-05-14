//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1305/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1305<F: Float>(t1008: F, t1701: F, t93271: F, t12411: F, t1354: F, t5813: F, t6604: F, t92557: F, t92433: F, t104689: F, t2001: F, t101406: F, t101410: F, t104692: F, t104742: F, t104925: F, t23715: F, t23847: F, t26692: F, t26745: F, t2992: F, t5785: F, t92429: F, t93169: F, t94821: F, t94823: F, t94827: F, t94836: F) -> (F, F) {
    let t105190 = t1701 * t93271 * t1008;
    let t105201 = t12411 * t1354;
    let t105207 = t5813 * t92557 * t6604;
    let t105211 = 0.17780800291358024692e0 * t5813 * t92433 * t6604;
    let t105212 = t2001 * t104689;
    let t105218 = 0.44452000728395061731e-1 * t94821 - 0.53706137268299704367e-1 * t94823 - 0.46992870109762241322e0 * t94827 + 0.1611184118048991131e0 * t94836 + 0.15303647250623035442e2 * t5785 * t105190 + 0.13335600218518518519e0 * t23715 * t93169 * t2992 * t104742 - 0.22226000364197530865e-1 * t26692 * t101406 - 0.51860667516460905352e-1 * t26692 * t101410 - 0.90613700826057446696e0 * t105201 * t26745 - 0.90613700826057446696e0 * t23847 * t104925 + 0.22226000364197530865e-1 * t105207 + t105211 - 0.18122740165211489339e1 * t105212 * t104692 - 0.97794401602469135802e0 * t5813 * t92429 * t6604;
    (t105190, t105218)
}
