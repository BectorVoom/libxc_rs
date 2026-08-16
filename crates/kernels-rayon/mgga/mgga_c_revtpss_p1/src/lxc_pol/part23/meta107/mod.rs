//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta107 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk704;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk705;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk706;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk707;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk708;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta107(t225: f64, t2735: f64, t826: f64, t849: f64, t820: f64, t823: f64, t843: f64, t839: f64, t241: f64, t72: f64, t853: f64, t245: f64, t231: f64, t775: f64, t213: f64, t860: f64, t256: f64, t866: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2736, t2737, t2739, t2741) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk704(t225, t2735, t826, t849, t820, t823, t843);
        let (t2742, t2745) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk705(t2741, t839, t241, t820, t823);
        let (t2746, t2747) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk706(t72, t853, t245);
        let t2749 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk707(t231, t775);
        let (t2765, t2769) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk708(t213, t860, t256, t866);
        let t2770 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk709(t225, t2769);
    (t2736, t2737, t2739, t2741, t2742, t2745, t2746, t2747, t2749, t2765, t2769, t2770)
}
