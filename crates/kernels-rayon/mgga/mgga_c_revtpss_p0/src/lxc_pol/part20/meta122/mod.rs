//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta122 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk704;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk705;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk706;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk707;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk708;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk709;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk710;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta122(t3361: f64, t2251: f64, t3360: f64, t128: f64, t2304: f64, t1120: f64, t1121: f64, t2258: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3362 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk704(t3361);
        let t3363 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk705(t2251, t3362);
        let (t3364, t3365) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk706(t3360, t3363, t128);
        let t3367 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk707(t2304);
        let t3368 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk708(t2251, t3367);
        let (t3369, t3370) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk709(t1120, t3368, t128);
        let t3372 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk710(t1121, t2258);
        let (t3373, t3374) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk711(t1120, t3372, t128);
    (t3362, t3363, t3364, t3365, t3367, t3368, t3369, t3370, t3372, t3373, t3374)
}
