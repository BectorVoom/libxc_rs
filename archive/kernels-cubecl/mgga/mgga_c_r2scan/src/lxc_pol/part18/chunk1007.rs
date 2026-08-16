//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1007/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1007<F: Float>(t3115: F, t3308: F, t10776: F, t3100: F, t10772: F, t10781: F, t3105: F, t261: F, t3191: F, t7628: F, t3182: F, t7614: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12476 = t3308 * t3115;
    let t12477 = t10776 * t12476;
    let t12479 = t3308 * t3100;
    let t12480 = t10772 * t12479;
    let t12482 = t10781 * t3105;
    let t12486 = t261 * t3191;
    let t12487 = t7628 * t12486;
    let t12489 = t261 * t3182;
    let t12490 = t7614 * t12489;
    (t12476, t12477, t12479, t12480, t12482, t12486, t12487, t12489, t12490)
}
