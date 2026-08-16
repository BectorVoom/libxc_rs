//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta825 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3073;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3074;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3075;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3076;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta825(t12256: f64, t3617: f64, t51957: f64, t51959: f64, t3362: f64, t482: f64, t12268: f64, t1263: f64, t43858: f64, t43865: f64, t43883: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t56212: f64, t56214: f64, t56216: f64, t56221: f64, t56226: f64, t56229: f64, t56230: f64, t56234: f64, t56236: f64, t448: f64, t56211: f64, t300: f64, t16784: f64, t3539: f64, t12230: f64, t5104: f64, t12227: f64, t3385: f64, t12357: f64, t3433: f64, t5108: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56246, t56248) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3073(t12256, t3617, t51957, t51959);
        let t56252 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3074(t3362, t482, t51957, t51959);
        let (t56254, t56256) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3075(t12268, t1263, t51957, t51959);
        let t56258 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3076(t43858, t43865, t43883, t43888, t43890, t43892, t43894, t43896, t56212, t56214, t56216, t56221, t56226, t56229, t56230, t56234, t56236, t56248, t56252, t56256);
        let (t56260, t56262, t56264, t56268, t56271) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3077(t448, t56211, t56258, t300, t16784, t3539, t12230, t5104, t12227, t3385, t12357, t3433, t5108);
    (t56246, t56248, t56252, t56254, t56256, t56260, t56262, t56264, t56268, t56271)
}
