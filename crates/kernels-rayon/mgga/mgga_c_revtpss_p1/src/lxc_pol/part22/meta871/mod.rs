//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta871 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3031;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3032;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta871(t1561: f64, t40360: f64, t14843: f64, t40864: f64, t10779: f64, t14931: f64, t1548: f64, t2724: f64, t10811: f64, t14693: f64, t2682: f64, t2719: f64, t4368: f64, t820: f64, t10778: f64, t221: f64, t10777: f64, t14792: f64, t2659: f64, t4503: f64, t816: f64, t14803: f64, t50769: f64, t4372: f64, t9784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51104, t51106, t51110, t51112, t51121) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3031(t1561, t40360, t14843, t40864, t10779, t14931, t1548, t2724, t10811, t14693, t2682, t2719, t4368, t820);
        let (t51123, t51125, t51133, t51135, t51168, t51170) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3032(t10778, t221, t10777, t14792, t2659, t4503, t816, t14803, t50769, t14931, t4372, t9784);
    (t51104, t51106, t51110, t51112, t51121, t51123, t51125, t51133, t51135, t51168, t51170)
}
