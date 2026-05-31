//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 566/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk566<F: Float>(t170: F, t3128: F, t159: F, t1650: F, t1662: F, t1667: F, t1671: F, t1688: F, t1695: F, t1702: F, t216: F, t2738: F, t2741: F, t2744: F, t2750: F, t3034: F, t3124: F, t41: F) -> (F, F) {
    let t3129 = t3128 * t170;
    let t3136 = -t41 * t3124 + t1650 - F::cast_from(0.21973736767207854065e-2_f64) * t3034 * t216 + F::cast_from(0.285764e-1_f64) * t159 * t3129 + F::cast_from(0.34631718211362927518e2_f64) * t2738 - t1662 + t1667 - t1671 + t1688 - F::cast_from(0.23392894490538584828e1_f64) * t2741 + F::cast_from(0.2701041328e0_f64) * t2744 - t1695 + F::cast_from(0.11696447245269292414e1_f64) * t2750 - t1702;
    (t3129, t3136)
}
