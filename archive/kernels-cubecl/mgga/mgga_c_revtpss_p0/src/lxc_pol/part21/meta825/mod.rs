//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta825 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3073;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3074;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3075;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3076;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta825<F: Float>(t12256: F, t3617: F, t51957: F, t51959: F, t3362: F, t482: F, t12268: F, t1263: F, t43858: F, t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t56212: F, t56214: F, t56216: F, t56221: F, t56226: F, t56229: F, t56230: F, t56234: F, t56236: F, t448: F, t56211: F, t300: F, t16784: F, t3539: F, t12230: F, t5104: F, t12227: F, t3385: F, t12357: F, t3433: F, t5108: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t56246, t56248) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3073::<F>(t12256, t3617, t51957, t51959);
        let t56252 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3074::<F>(t3362, t482, t51957, t51959);
        let (t56254, t56256) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3075::<F>(t12268, t1263, t51957, t51959);
        let t56258 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3076::<F>(t43858, t43865, t43883, t43888, t43890, t43892, t43894, t43896, t56212, t56214, t56216, t56221, t56226, t56229, t56230, t56234, t56236, t56248, t56252, t56256);
        let (t56260, t56262, t56264, t56268, t56271) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3077::<F>(t448, t56211, t56258, t300, t16784, t3539, t12230, t5104, t12227, t3385, t12357, t3433, t5108);
    (t56246, t56248, t56252, t56254, t56256, t56260, t56262, t56264, t56268, t56271)
}
