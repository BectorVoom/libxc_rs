//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta868 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2764;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta868(t22025: f64, t2661: f64, t5675: f64, t9934: f64, t6836: f64, t9940: f64, t1353: f64, t13767: f64, t13768: f64, t5591: f64, t21969: f64, t221: f64, t3978: f64, t3979: f64, t4010: f64, t6816: f64, t22027: f64, t9775: f64, t22252: f64, t3992: f64, t543: f64, t550: f64, t22263: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73985, t73994, t73998, t74010) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2764(t22025, t2661, t5675, t9934, t6836, t9940, t1353, t13767, t13768, t5591, t21969, t221, t3978, t3979);
        let (t74012, t74015, t74017, t74022, t74024) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2765(t4010, t6816, t1353, t13767, t2661, t22027, t9775, t22252, t3992, t543, t550, t22263);
    (t73985, t73994, t73998, t74010, t74012, t74015, t74017, t74022, t74024)
}
