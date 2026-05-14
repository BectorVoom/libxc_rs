//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 710/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk710<F: Float>(t2132: F, t2183: F, t296: F, t297: F, t306: F, t307: F, t6101: F, t1275: F, t815: F, t817: F, t312: F, t317: F, t6100: F, t832: F, t325: F, t1347: F, t349: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6583 = t2183 * t2132;
    let t6621 = 1.0 / t297 / t296;
    let t6635 = 1.0 / t307 / t306;
    let t6648 = 154.0 / 27.0 * t6101;
    let t6654 = t815 * t1275;
    let t6659 = t817 * t817;
    let t6660 = 1.0 / t6659;
    let t6661 = t312 * t6660;
    let t6678 = 154.0 / 27.0 * t317 * t6100;
    let t6691 = t832 * t832;
    let t6692 = 1.0 / t6691;
    let t6693 = t325 * t6692;
    let t6755 = 1.0 / t1347 / t349;
    (t6583, t6621, t6635, t6648, t6654, t6659, t6660, t6661, t6678, t6691, t6692, t6693, t6755)
}
