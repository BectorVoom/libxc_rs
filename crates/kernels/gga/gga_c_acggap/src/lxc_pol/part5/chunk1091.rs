//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1091/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1091<F: Float>(t20174: F, t944: F, t3409: F, t6245: F, t1150: F, t1181: F, t13364: F, t16992: F, t17139: F, t175: F, t22102: F, t22105: F, t22107: F, t22112: F, t22114: F, t22120: F, t22125: F, t336: F, t398: F, t418: F, t4643: F, t4735: F, t5012: F, t5116: F, t525: F, t5630: F, t922: F, t942: F) -> (F, F) {
    let t22127 = t20174 * t944;
    let t22132 = t3409 * t6245;
    let t22134 = -0.34299214494455789578e-2 * t418 * t398 * t4643 * t5116 + 0.17149607247227894789e-2 * t22102 - 0.68026775414003982663e-1 * t16992 + 0.34299214494455789578e-2 * t22105 - 0.34299214494455789578e-1 * t17139 * t13364 * t525 * t22107 + 7.0 / 72.0 * t22112 + 7.0 / 72.0 * t22114 + t1150 * t336 * t5630 * t922 / 16.0 - 0.20579528696673473747e-1 * t4735 * t1181 * t22120 * t5012 - 35.0 / 432.0 * t22125 + 0.85748036236139473944e-3 * t942 * t398 * t175 * t22127 - 0.40015750243531754508e-2 * t22132;
    (t22127, t22134)
}
