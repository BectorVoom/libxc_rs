//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta791 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2607;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2608;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta791(t18495: f64, t2652: f64, t18500: f64, t18493: f64, t221: f64, t2674: f64, t40683: f64, t18441: f64, t9775: f64, t18437: f64, t2661: f64, t2662: f64, t4352: f64, t4424: f64, t18413: f64, t837: f64, t10716: f64, t18402: f64, t10722: f64, t5993: f64, t18481: f64, t50768: f64, t51176: f64, t18333: f64, t50769: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61630, t61632, t61641, t61645, t61660, t61669) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2607(t18495, t2652, t18500, t18493, t221, t2674, t40683, t18441, t9775, t18437, t2661, t2662, t4352, t4424);
        let (t61673, t61675, t61677, t61689, t61692) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2608(t18413, t2661, t2662, t837, t10716, t18402, t10722, t5993, t18481, t50768, t51176, t18333, t50769);
    (t61630, t61632, t61641, t61645, t61660, t61669, t61673, t61675, t61677, t61689, t61692)
}
