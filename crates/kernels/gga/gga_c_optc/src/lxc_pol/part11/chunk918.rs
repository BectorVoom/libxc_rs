//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 918/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk918<F: Float>(t17239: F, t914: F, t14763: F, t14767: F, t14774: F, t14779: F, t14783: F, t17212: F, t17216: F, t17220: F, t17223: F, t17226: F, t17229: F, t17232: F, t17236: F, t2721: F, t2812: F, t3907: F, t3917: F, t8134: F, t930: F) -> F {
    let t17240 = t914 * t17239;
    let t17243 = F::new(0.1169609647897054359e2) * t14763 - F::new(0.15486228121497046737e2) * t14767 + F::new(0.4645868436449114021e2) * t14774 + F::new(0.28977204965962526182e-1) * t14779 + F::new(0.16793568152788065762e-1) * t14783 + F::new(0.8790987341241436962e3) * t3917 * t17212 + F::new(0.4645868436449114021e2) * t3907 * t17216 + F::new(0.1169609647897054359e2) * t2812 * t17220 + F::new(0.1949349413161757265e2) * t2812 * t17223 + F::new(0.11360101276506094136e1) * t2721 * t17226 + F::new(0.11360101276506094136e1) * t2721 * t17229 - F::new(0.8987985586528718635e4) * t8134 * t17232 - F::new(0.17386322979577515709e0) * t930 * t17236 + F::new(0.11590881986385010473e0) * t930 * t17240;
    t17243
}
