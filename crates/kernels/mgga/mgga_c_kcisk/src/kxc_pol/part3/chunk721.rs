//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 721/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk721<F: Float>(t772: F, t5464: F, t1772: F, t10487: F, t786: F, t10441: F, t5006: F, t2023: F, t5507: F, t5515: F, t7261: F, t10449: F, t2014: F, t1775: F, t3293: F, t5491: F, t12143: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t783 = 0.0 < t772;
    let t12194 = t5464 * sigma2;
    let t12195 = t12194 * t1772;
    let t12198 = t786 * t10487;
    let t12199 = t12198 * t10441;
    let t12200 = t5006 * t12199;
    let t12203 = t5507 * t2023;
    let t12204 = t12203 * t5515;
    let t12205 = t7261 * t12204;
    let t12208 = t2014 * t10449;
    let t12209 = t1775 * t12208;
    let t12214 = t3293 * t2023;
    let t12215 = t5491 * t12214;
    let t12216 = t1775 * t12215;
    let t12220 = piecewise3(t783, t12143, -t12143);
    (t12195, t12200, t12205, t12209, t12216, t12220)
}
