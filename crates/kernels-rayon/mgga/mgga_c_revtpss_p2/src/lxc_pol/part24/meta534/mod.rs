//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1573;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1574;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta534(t22837: f64, t9962: f64, t22860: f64, t47194: f64, t22849: f64, t3957: f64, t13790: f64, t22020: f64, t2661: f64, t9934: f64, t177: f64, t22789: f64, t762: f64, t72: f64, t757: f64, t1317: f64, t22790: f64, t1320: f64, t512: f64, t749: f64, t221: f64, t22954: f64, t4018: f64, t4019: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t85839, t85865, t85873, t85885, t85895) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1573(t22837, t9962, t22860, t47194, t22849, t3957, t13790, t22020, t2661, t9934, t177, t22789, t762);
        let (t85912, t85929, t85931, t85986, t86061) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1574(t22789, t72, t757, t1317, t22790, t1320, t512, t749, t221, t22954, t4018, t4019);
    (t85839, t85865, t85873, t85885, t85895, t85912, t85929, t85931, t85986, t86061)
}
