//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1031/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1031<F: Float>(t128: F, t20569: F, t647: F, t1030: F, t34106: F, t33273: F, t9053: F, t11484: F, t1688: F, t20897: F, t11313: F, t26887: F, t11311: F, t5987: F, t11312: F, t3064: F, t3949: F) -> (F, F, F, F, F, F, F, F) {
    let t34711 = t20569 * t647 * t128;
    let t34712 = t1030 * t34106 * t34711;
    let t34714 = t1030 * t33273;
    let t34715 = t34714 * t9053;
    let t34718 = t11484 * t1688 * t20897;
    let t34720 = t26887 * t11313;
    let t34723 = t5987 * t11311 * t11313;
    let t34726 = t11312 * t3064 * t3949;
    (t34711, t34712, t34714, t34715, t34718, t34720, t34723, t34726)
}
