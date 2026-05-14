//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 744/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk744<F: Float>(t2917: F, t840: F, t2920: F, t55: F, t12535: F, t2879: F, t2885: F, t2887: F, t829: F, t142: F, t2855: F, t2858: F, t298: F, t56: F, t69: F, t918: F) -> (F, F, F, F, F, F) {
    let t12610 = 1.0 / t2917 / t840;
    let t12613 = 1.0 / t2920 / t55;
    let t12614 = t12610 * t12535 * t12613;
    let t12620 = 0.48245472966453314466e2 * t2885 * t2879 * t2887 * t829;
    let t12624 = 0.10685e0 * t298 * t142 * t2855 * t2858;
    let t12626 = t69 * t918 * t56;
    (t12610, t12613, t12614, t12620, t12624, t12626)
}
