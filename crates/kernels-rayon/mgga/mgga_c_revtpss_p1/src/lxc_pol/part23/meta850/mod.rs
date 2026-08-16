//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta850 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2733;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2734;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta850(t1250: f64, t5245: f64, t1794: f64, t372: f64, t5277: f64, t17395: f64, t17400: f64, t20809: f64, t12772: f64, t21172: f64, t5331: f64, t3655: f64, t6598: f64, t6602: f64, t20816: f64, t3708: f64, t17384: f64, t17448: f64, t17183: f64, t17350: f64, t5436: f64, t17435: f64, t5323: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71055, t71061, t71081, t71112, t71117, t71187) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2733(t1250, t5245, t1794, t372, t5277, t17395, t17400, t20809, t12772, t21172, t5331, t3655, t6598);
        let (t71192, t71207, t71232, t71238, t71275, t71278) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2734(t3655, t6602, t20816, t3708, t17384, t17448, t17183, t17350, t17395, t5436, t17435, t5323);
    (t71055, t71061, t71081, t71112, t71117, t71187, t71192, t71207, t71232, t71238, t71275, t71278)
}
