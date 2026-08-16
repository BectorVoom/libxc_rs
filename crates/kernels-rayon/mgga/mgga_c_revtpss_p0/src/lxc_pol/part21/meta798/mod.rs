//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta798 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2890;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2891;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta798(t15475: f64, t2869: f64, t11385: f64, t1609: f64, t11387: f64, t2918: f64, t934: f64, t41578: f64, t4636: f64, t11528: f64, t15380: f64, t11294: f64, t15390: f64, t2874: f64, t15474: f64, t2924: f64, t2926: f64, t11300: f64, t4635: f64, t2873: f64, t4587: f64, t2876: f64, t11298: f64, t1596: f64, t11301: f64, t11466: f64, t1633: f64, t11299: f64, t11116: f64, t11525: f64, t11551: f64, t11557: f64, t15350: f64, t15406: f64, t52137: f64, t965: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52481, t52486, t52488, t52490, t52492) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2890(t15475, t2869, t11385, t1609, t11387, t2918, t934, t41578, t4636, t11528, t15380, t11294, t15390);
        let (t52495, t52499, t52502, t52507) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2891(t15475, t2874, t934, t15474, t2924, t2926, t11300, t11385, t4635, t2873, t4587, t2876);
        let (t52510, t52516, t52520) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2892(t11298, t1596, t11301, t11466, t1633, t11299, t1609, t11116, t11525, t11551, t11557, t15350, t15406, t52137, t52481, t52486, t52488, t52490, t52492, t52495, t52499, t52502, t52507, t965, t973);
    (t52481, t52486, t52488, t52490, t52492, t52495, t52499, t52502, t52507, t52510, t52516, t52520)
}
