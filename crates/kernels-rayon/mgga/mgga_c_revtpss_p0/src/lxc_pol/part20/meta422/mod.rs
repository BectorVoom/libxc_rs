//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1581;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1582;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1583;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1584;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1585;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1586;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1587;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta422(t2435: f64, t3373: f64, t3369: f64, t12313: f64, t689: f64, t12319: f64, t128: f64, t3360: f64, t43789: f64, t1120: f64, t43793: f64, t43797: f64, t43854: f64, t43881: f64, t43883: f64, t43886: f64, t43888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t43890 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1581(t2435, t3373);
        let t43892 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1582(t2435, t3369);
        let t43894 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1583(t12313, t689);
        let t43896 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1584(t12319, t689);
        let t43899 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1585(t128, t3360, t43789);
        let t43902 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1586(t1120, t128, t43793);
        let t43905 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1587(t1120, t128, t43797);
        let t43907 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1588(t43854, t43881, t43883, t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905);
    (t43890, t43892, t43894, t43896, t43899, t43902, t43905, t43907)
}
