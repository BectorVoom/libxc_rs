//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta862 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2752;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta862(t2608: f64, t512: f64, t6800: f64, t177: f64, t21931: f64, t762: f64, t1320: f64, t22193: f64, t22461: f64, t4147: f64, t749: f64, t22212: f64, t2516: f64, t72: f64, t757: f64, t6922: f64, t9593: f64, t22185: f64, t2619: f64, t22404: f64, t3920: f64, t1445: f64, t22445: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73350, t73352, t73374, t73407, t73476, t73481) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2752(t2608, t512, t6800, t177, t21931, t762, t1320, t22193, t22461, t4147, t749, t22212, t2516);
        let (t73493, t73499, t73515, t73587, t73590) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2753(t21931, t72, t757, t6922, t9593, t22185, t2619, t22404, t3920, t1445, t22445, t689);
    (t73350, t73352, t73374, t73407, t73476, t73481, t73493, t73499, t73515, t73587, t73590)
}
