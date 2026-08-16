//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 918/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk918(t17239: f64, t914: f64, t14763: f64, t14767: f64, t14774: f64, t14779: f64, t14783: f64, t17212: f64, t17216: f64, t17220: f64, t17223: f64, t17226: f64, t17229: f64, t17232: f64, t17236: f64, t2721: f64, t2812: f64, t3907: f64, t3917: f64, t8134: f64, t930: f64) -> f64 {
    let t17240 = t914 * t17239;
    let t17243 = 0.1169609647897054359e2_f64 * t14763 - 0.15486228121497046737e2_f64 * t14767 + 0.4645868436449114021e2_f64 * t14774 + 0.28977204965962526182e-1_f64 * t14779 + 0.16793568152788065762e-1_f64 * t14783 + 0.8790987341241436962e3_f64 * t3917 * t17212 + 0.4645868436449114021e2_f64 * t3907 * t17216 + 0.1169609647897054359e2_f64 * t2812 * t17220 + 0.1949349413161757265e2_f64 * t2812 * t17223 + 0.11360101276506094136e1_f64 * t2721 * t17226 + 0.11360101276506094136e1_f64 * t2721 * t17229 - 0.8987985586528718635e4_f64 * t8134 * t17232 - 0.17386322979577515709e0_f64 * t930 * t17236 + 0.11590881986385010473e0_f64 * t930 * t17240;
    t17243
}
