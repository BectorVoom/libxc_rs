//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta157 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1002;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1003;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta157(t3670: f64, t480: f64, t3568: f64, t482: f64, t371: f64, t372: f64, t1236: f64, t127: f64, t1235: f64, t221: f64, t462: f64, t696: f64, t461: f64, t1226: f64, t140: f64, t1222: f64, t1225: f64, t2258: f64, t1012: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3671, t3672, t3674) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1002(t3670, t480, t3568, t482, t371, t372);
        let t3678 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1003(t1236, t127, t371);
        let (t3679, t3682, t3684, t3685, t3686, t3688, t3689) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1004(t1235, t3678, t221, t462, t696, t461, t1226, t140, t1222, t1225, t2258, t1012);
    (t3671, t3672, t3674, t3678, t3679, t3682, t3684, t3685, t3686, t3688, t3689)
}
