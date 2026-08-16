//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1477;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta485(t3670: f64, t6594: f64, t3718: f64, t44546: f64, t6689: f64, t3717: f64, t70994: f64, t3617: f64, t6587: f64, t3147: f64, t6593: f64, t3594: f64, t3597: f64, t1244: f64, t17628: f64, t5373: f64, t3655: f64, t6595: f64, t1222: f64, t6658: f64, t697: f64, t6662: f64, t1209: f64, t1284: f64, t6695: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71280, t71294, t71513, t71543, t71691, t71693) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1477(t3670, t6594, t3718, t44546, t6689, t3717, t70994, t3617, t6587, t3147, t6593, t3594, t3597);
        let (t71699, t71718, t71744, t71928, t71931, t72267) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1478(t1244, t3594, t71691, t17628, t5373, t3655, t6595, t1222, t6658, t697, t6662, t1209, t1284, t6695);
    (t71280, t71294, t71513, t71543, t71693, t71699, t71718, t71744, t71928, t71931, t72267)
}
