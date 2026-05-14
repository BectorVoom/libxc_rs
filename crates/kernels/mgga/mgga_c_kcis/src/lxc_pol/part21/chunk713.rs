//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 713/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk713<F: Float>(t2308: F, t2311: F, t237: F, t88: F, t2333: F, t2339: F, t2341: F, t661: F, t2371: F, t52: F, t2375: F, t8656: F, t12: F, t3: F, t160: F, t2326: F, t8581: F) -> (F, F, F, F, F, F) {
    let t8674 = 0.10685e0 * t237 * t88 * t2308 * t2311;
    let t8678 = 0.48245472966453314466e2 * t2339 * t2333 * t2341 * t661;
    let t8680 = 1.0 / t2371 / t52;
    let t8682 = t8680 * t8656 * t2375;
    let t8689 = 1.0/pow_3_2(t12);
    let t8690 = t8689 * t3;
    let t8691 = t8690 * t160;
    let t8693 = t2326 * t8581;
    (t8674, t8678, t8680, t8682, t8691, t8693)
}
