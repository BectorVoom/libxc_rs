//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 668/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk668(t11628: f64, t1457: f64, t11576: f64, t808: f64, t568: f64, t11577: f64, t531: f64, t3645: f64, t769: f64, t314: f64, t313: f64, t10019: f64, t11058: f64, t2049: f64, t2194: f64, t2197: f64, t317: f64, t3655: f64, t3658: f64, t3662: f64, t3667: f64, t3671: f64, t5771: f64, t6141: f64, t797: f64, t813: f64) -> (f64, f64) {
    let t11908 = t1457 * t11628;
    let t11915 = t808 * t11576;
    let t11916 = t568 * t11915;
    let t11923 = t531 * t11577;
    let t11928 = t769 * t3645;
    let t11931 = t314 * t11576;
    let t11932 = t313 * t11931;
    let t11935 = -0.15337170381568299871e1_f64 * t11058 + 0.31952438294933958063e-1_f64 * t10019 - 0.71500979903700853338e0_f64 * t6141 * t11908 + 0.11502877786176224903e2_f64 * t2197 * t3667 - 0.23005755572352449806e1_f64 * t2194 * t3671 - 0.23005755572352449806e1_f64 * t813 * t11916 + 0.71500979903700853338e0_f64 * t5771 * t3655 - 0.35750489951850426669e0_f64 * t2049 * t3658 - 0.35750489951850426669e0_f64 * t797 * t11923 - 0.46011511144704899612e1_f64 * t2194 * t3662 + 0.35750489951850426669e0_f64 * t11928 * t317 + 0.35750489951850426669e0_f64 * t11932 * t317;
    (t11931, t11935)
}
