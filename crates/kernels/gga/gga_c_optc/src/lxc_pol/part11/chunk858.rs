//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 858/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk858<F: Float>(t17219: F, t2813: F, t17134: F, t3836: F, t2722: F, t17169: F, t17148: F, t2674: F, t16917: F, t2601: F, t914: F, t2633: F, t14763: F, t14767: F, t14774: F, t14779: F, t14783: F, t17212: F, t17216: F, t2721: F, t2812: F, t3907: F, t3917: F, t8134: F, t930: F) -> (F, F, F) {
    let t17220 = t2813 * t17219;
    let t17223 = t3836 * t17134;
    let t17226 = t2722 * t17219;
    let t17229 = t2722 * t17169;
    let t17232 = t17148 * t2674;
    let t17235 = t2601 * t16917;
    let t17236 = t914 * t17235;
    let t17239 = t2633 * t16917;
    let t17240 = t914 * t17239;
    let t17243 = 0.1169609647897054359e2 * t14763 - 0.15486228121497046737e2 * t14767 + 0.4645868436449114021e2 * t14774 + 0.28977204965962526182e-1 * t14779 + 0.16793568152788065762e-1 * t14783 + 0.8790987341241436962e3 * t3917 * t17212 + 0.4645868436449114021e2 * t3907 * t17216 + 0.1169609647897054359e2 * t2812 * t17220 + 0.1949349413161757265e2 * t2812 * t17223 + 0.11360101276506094136e1 * t2721 * t17226 + 0.11360101276506094136e1 * t2721 * t17229 - 0.8987985586528718635e4 * t8134 * t17232 - 0.17386322979577515709e0 * t930 * t17236 + 0.11590881986385010473e0 * t930 * t17240;
    (t17235, t17239, t17243)
}
