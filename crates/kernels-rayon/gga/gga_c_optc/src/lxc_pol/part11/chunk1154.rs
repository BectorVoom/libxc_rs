//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1154/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1154(t51450: f64, t8214: f64, t8216: f64, t8208: f64, t8210: f64, t10856: f64, t2668: f64, t4963: f64, t17125: f64, t2812: f64, t8143: f64, t50765: f64, t953: f64) -> (f64, f64, f64, f64, f64) {
    let t51452 = t8214 * t51450 * t8216;
    let t51461 = t8208 * t51450 * t8210;
    let t51502 = t2668 * t10856 * t4963;
    let t51515 = t2812 * t8143 * t17125;
    let t51564 = t953 * t50765;
    (t51452, t51461, t51502, t51515, t51564)
}
