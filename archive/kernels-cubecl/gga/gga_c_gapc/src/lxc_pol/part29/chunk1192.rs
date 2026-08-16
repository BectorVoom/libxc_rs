//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1192/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1192<F: Float>(t11311: F, t11313: F, t5987: F, t11312: F, t3064: F, t3949: F, t1036: F, t13790: F, t1649: F, t19677: F, t33273: F, t11387: F, t5248: F, t5553: F) -> (F, F, F, F, F) {
    let t34723 = t5987 * t11311 * t11313;
    let t34726 = t11312 * t3064 * t3949;
    let t34729 = t11312 * t1036 * t13790;
    let t34732 = t19677 * t33273 * t1649;
    let t34735 = t5553 * t11387 * t5248;
    (t34723, t34726, t34729, t34732, t34735)
}
