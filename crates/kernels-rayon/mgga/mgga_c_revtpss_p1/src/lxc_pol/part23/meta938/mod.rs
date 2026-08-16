//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta938 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3083;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3084;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta938(t1145: f64, t141: f64, t81207: f64, t3417: f64, t81169: f64, t81173: f64, t12254: f64, t81165: f64, t56176: f64, t81439: f64, t81442: f64, t81445: f64, t81448: f64, t81451: f64, t81454: f64, t81457: f64, t56183: f64, t56236: f64, t58536: f64, t68389: f64, t68399: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t44919: f64, t52011: f64, t77513: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t81460, t81463, t81466, t81469, t81472) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3083(t1145, t141, t81207, t3417, t81169, t81173, t12254, t81165, t56176, t81439, t81442, t81445, t81448, t81451, t81454, t81457);
        let (t81485, t81489) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3084(t56183, t56236, t58536, t68389, t68399, t81224, t81228, t81230, t81232, t81234, t81236, t44919, t52011, t77513);
    (t81460, t81463, t81466, t81469, t81472, t81485, t81489)
}
