//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 880/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk880<F: Float>(t11678: F, t2147: F, t3344: F, t980: F, t10760: F, t8089: F, t6535: F, t261: F, t2726: F, t3299: F, t2593: F, t3295: F, t2599: F, t3308: F, t1577: F, t10710: F, t7257: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11679 = t2147 * t11678;
    let t11681 = t980 * t3344;
    let t11683 = t10760 * t8089;
    let t11684 = t6535 * t11683;
    let t11686 = t261 * t2726;
    let t11687 = t3299 * t11686;
    let t11691 = t3295 * t2593;
    let t11693 = t3308 * t2599;
    let t11694 = t1577 * t11693;
    let t11696 = t10710 * t7257;
    (t11679, t11681, t11683, t11684, t11686, t11687, t11691, t11693, t11694, t11696)
}
