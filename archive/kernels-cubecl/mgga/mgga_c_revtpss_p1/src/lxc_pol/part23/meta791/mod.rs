//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta791 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2607;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2608;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta791<F: Float>(t18495: F, t2652: F, t18500: F, t18493: F, t221: F, t2674: F, t40683: F, t18441: F, t9775: F, t18437: F, t2661: F, t2662: F, t4352: F, t4424: F, t18413: F, t837: F, t10716: F, t18402: F, t10722: F, t5993: F, t18481: F, t50768: F, t51176: F, t18333: F, t50769: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t61630, t61632, t61641, t61645, t61660, t61669) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2607::<F>(t18495, t2652, t18500, t18493, t221, t2674, t40683, t18441, t9775, t18437, t2661, t2662, t4352, t4424);
        let (t61673, t61675, t61677, t61689, t61692) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2608::<F>(t18413, t2661, t2662, t837, t10716, t18402, t10722, t5993, t18481, t50768, t51176, t18333, t50769);
    (t61630, t61632, t61641, t61645, t61660, t61669, t61673, t61675, t61677, t61689, t61692)
}
