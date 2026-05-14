//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 720/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk720<F: Float>(t10588: F, t10621: F, t845: F, t91: F, t305: F, t631: F, t7242: F, t798: F, t898: F, t2756: F, t856: F, t10246: F, t10279: F, t10282: F, t10259: F, t10265: F, t10269: F, t10273: F, t10391: F, t10552: F, t10553: F, t10555: F) -> (F, F, F, F, F) {
    let t10622 = t10588 + t10621;
    let t10624 = t91 * t845 * t10622;
    let t10631 = 1.0 / t305 / t631 / t898 / t798 / t7242 / 4.0;
    let t10632 = t2756 * t856;
    let t10634 = t91 * t10631 * t10632;
    let t10636 = 2.0 / 9.0 * t10246;
    let t10640 = 4.0 / 27.0 * t10279;
    let t10641 = t10282 / 9.0;
    let t10642 = -t10391 / 3.0 + t10552 - t10553 - 2.0 * t10265 - t10555 + t10624 / 6.0 + t10634 / 8.0 - t10636 - t10259 / 9.0 + 2.0 * t10269 - 10.0 / 81.0 * t10273 - t10640 + t10641;
    (t10622, t10624, t10631, t10634, t10642)
}
