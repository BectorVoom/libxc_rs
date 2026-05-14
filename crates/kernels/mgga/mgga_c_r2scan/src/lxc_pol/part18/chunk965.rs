//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 965/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk965<F: Float>(t38175: F, t10841: F, t1607: F, t2228: F, t505: F, t539: F, t252: F, t3320: F, t6262: F, t783: F, t10949: F, t10992: F, t2315: F, t3446: F, t23194: F, t263: F, t3438: F, t6874: F) -> (F, F, F, F, F, F, F) {
    let t38176 = 0.174549769648958674e0 * t38175;
    let t38177 = t10841 * t1607;
    let t38182 = t2228 * t505;
    let t38183 = t38182 * t539;
    let t38189 = t783 * t252 * t6262 * t3320;
    let t38190 = 0.23080304851772712107e1 * t38189;
    let t38211 = t3446 * t10992 * t10949 * t2315;
    let t38225 = t3446 * t263 * t23194 * t3438 * t6874;
    (t38176, t38177, t38182, t38183, t38190, t38211, t38225)
}
