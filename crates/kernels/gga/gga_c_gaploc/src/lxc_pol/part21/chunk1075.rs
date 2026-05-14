//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1075/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1075<F: Float>(t32333: F, t21476: F, t2932: F, t7313: F, t24321: F, t2558: F, t9647: F, t1843: F, t24478: F, t7064: F, t10627: F, t161: F, t1845: F, t21488: F, t320: F, t795: F) -> (F, F, F, F, F, F, F) {
    let t32334 = 0.25635087433807414279e-2 * t32333;
    let t32336 = t21476 * t2932 * t7313;
    let t32337 = 0.64087718584518535698e-3 * t32336;
    let t32339 = t9647 * t24321 * t2558;
    let t32340 = 0.32043859292259267849e-3 * t32339;
    let t32342 = t7064 * t1843 * t24478;
    let t32343 = 0.32043859292259267849e-3 * t32342;
    let t32348 = t10627 * t161;
    let t32349 = t32348 * t1845;
    let t32351 = 0.11963040802443459997e-1 * t21488 * t320 * t795 * t32349;
    (t32334, t32337, t32340, t32343, t32348, t32349, t32351)
}
