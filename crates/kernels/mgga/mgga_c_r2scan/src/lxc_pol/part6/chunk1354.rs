//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1354/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1354<F: Float>(t6152: F, t7360: F, t20305: F, t24161: F, t6086: F, t6322: F, t980: F, t20758: F, t2627: F, t2631: F, t6240: F, t20822: F, t20826: F, t20830: F, t20834: F, t2139: F, t24300: F, t495: F, t5108: F, t5109: F, t6149: F, t6583: F, t8094: F, t8107: F, t8213: F, t8218: F) -> (F,) {
    let t25638 = t6152 * t7360;
    let t25660 = t20305 * t6086 * t24161;
    let t25662 = t980 * t6322;
    let t25664 = t20758 * t2627;
    let t25665 = 0.38415120233790484326e1 * t25664;
    let t25666 = t6240 * t2631;
    let t25667 = 0.64025200389650807209e0 * t25666;
    let t25668 = -0.20803732176130244552e1 * t25638 + 0.39006997830244208535e0 * t6152 * t8107 + 0.17348729279022588207e-2 * t20822 - 0.17465477326173296717e-1 * t20826 - 0.29272321618148349056e-1 * t20830 - 0.4075278042773769234e0 * t20834 + 0.39006997830244208535e0 * t2139 * t5109 * t24300 - 0.26004665220162805689e0 * t6583 * t5109 * t8213 * t495 - 0.39006997830244208535e0 * t5108 * t5109 * t8218 * t495 + 0.26004665220162805689e0 * t6149 * t8094 - 0.1047928639570397803e0 * t25660 + 0.41530324072742201648e-1 * t25662 + t25665 - t25667;
    (t25668,)
}
