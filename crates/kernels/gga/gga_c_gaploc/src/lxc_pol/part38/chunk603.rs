//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 603/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk603<F: Float>(t11628: F, t1457: F, t11576: F, t808: F, t568: F, t11577: F, t531: F, t3645: F, t769: F, t314: F, t313: F, t10019: F, t11058: F, t2049: F, t2194: F, t2197: F, t317: F, t3655: F, t3658: F, t3662: F, t3667: F, t3671: F, t5771: F, t6141: F, t797: F, t813: F) -> (F, F) {
    let t11908 = t1457 * t11628;
    let t11915 = t808 * t11576;
    let t11916 = t568 * t11915;
    let t11923 = t531 * t11577;
    let t11928 = t769 * t3645;
    let t11931 = t314 * t11576;
    let t11932 = t313 * t11931;
    let t11935 = -0.15337170381568299871e1 * t11058 + 0.31952438294933958063e-1 * t10019 - 0.71500979903700853338e0 * t6141 * t11908 + 0.11502877786176224903e2 * t2197 * t3667 - 0.23005755572352449806e1 * t2194 * t3671 - 0.23005755572352449806e1 * t813 * t11916 + 0.71500979903700853338e0 * t5771 * t3655 - 0.35750489951850426669e0 * t2049 * t3658 - 0.35750489951850426669e0 * t797 * t11923 - 0.46011511144704899612e1 * t2194 * t3662 + 0.35750489951850426669e0 * t11928 * t317 + 0.35750489951850426669e0 * t11932 * t317;
    (t11931, t11935)
}
