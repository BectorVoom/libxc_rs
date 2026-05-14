//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1319/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1319<F: Float>(t1882: F, t27036: F, t2185: F, t23657: F, t3408: F, t590: F, t5900: F, t12645: F, t5899: F, t9432: F, t11604: F, t23909: F, t27072: F, t27142: F, t23658: F, t27165: F) -> (F, F, F, F, F, F, F) {
    let t105482 = t1882 * t27036;
    let t105483 = 2.0 / 9.0 * t105482;
    let t105487 = t23657 * t2185 * t5900 * t3408 * t590;
    let t105491 = t5899 * t9432 * t5900 * t12645;
    let t105493 = t23909 * t11604;
    let t105495 = t27142 * t27072 * t105493;
    let t105499 = t23657 * t2185 * t27165 * t23658;
    (t105482, t105483, t105487, t105491, t105493, t105495, t105499)
}
