//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1503;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1504;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta500(t1558: f64, t231: f64, t6016: f64, t2782: f64, t2797: f64, t23167: f64, t251: f64, t2783: f64, t76131: f64, t18719: f64, t51549: f64, t23245: f64, t2798: f64, t686: f64, t72: f64, t23359: f64, t874: f64, t10871: f64, t4500: f64, t62808: f64, t125: f64, t23148: f64, t23244: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76163, t76169, t76172, t76182, t76206, t76223) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1503(t1558, t231, t6016, t2782, t2797, t23167, t251, t2783, t76131, t18719, t51549, t23245, t2798, t686, t72);
        let (t76237, t76242, t76255, t76279, t76284, t76289) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1504(t23359, t686, t72, t874, t10871, t6016, t4500, t62808, t125, t23148, t23167, t23244);
    (t76163, t76169, t76172, t76182, t76206, t76223, t76237, t76242, t76255, t76279, t76284, t76289)
}
