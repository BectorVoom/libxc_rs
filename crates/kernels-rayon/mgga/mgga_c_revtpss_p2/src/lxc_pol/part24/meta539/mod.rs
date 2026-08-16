//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1585;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1586;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta539(t1892: f64, t6861: f64, t6843: f64, t1385: f64, t22964: f64, t5741: f64, t75251: f64, t2782: f64, t4086: f64, t543: f64, t86455: f64, t14192: f64, t86445: f64, t9994: f64, t545: f64, t689: f64, t869: f64, t4003: f64, t5744: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86470, t86506, t86552, t86563, t86575, t86582, t86586) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1585(t1892, t6861, t6843, t1385, t22964, t5741, t75251, t2782, t4086, t543, t86455, t14192, t86445, t9994);
        let (t86597, t86604, t86608, t86634) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1586(t22964, t545, t689, t869, t2782, t4086, t543, t86506, t86445, t4003, t5744, t86470);
    (t86506, t86552, t86563, t86575, t86582, t86586, t86597, t86604, t86608, t86634)
}
