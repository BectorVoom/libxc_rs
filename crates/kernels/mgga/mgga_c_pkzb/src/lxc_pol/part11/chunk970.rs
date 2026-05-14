//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 970/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk970<F: Float>(t2590: F, t5256: F, t1717: F, t5295: F, t173: F, t5286: F, t588: F, t603: F, t1726: F, t5389: F, t158: F, t165: F, t5387: F, t1721: F, t1511: F, t5331: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16399 = t2590 * t5256;
    let t16402 = t1717 * t5295;
    let t16405 = t5286 * t173;
    let t16406 = t588 * t16405;
    let t16407 = t16406 * t603;
    let t16416 = t5389 * t1726;
    let t16421 = t158 / t5387 / t165;
    let t16425 = t1721 * t1721;
    let t16476 = t1511 * t5331;
    (t16399, t16402, t16405, t16406, t16407, t16416, t16421, t16425, t16476)
}
