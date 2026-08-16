//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta536 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2071;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2072;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2073;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta536(t30: f64, t21881: f64, t508: f64, t1518: f64, t5517: f64, t13584: f64, t9375: f64, t6785: f64, t9335: f64, t3833: f64, t5824: f64, t18280: f64, t2255: f64, t513: f64, t5549: f64, t605: f64, zeta_threshold: f64, t33: f64, t6792: f64, t9350: f64, t3841: f64, t6416: f64, t1113: f64, t20256: f64, t516: f64, t5557: f64, t162: f64, t187: f64, t1450: f64, t6922: f64, t9605: f64, t3874: f64, t1344: f64, t5574: f64, t9617: f64, t3881: f64, t1348: f64, t5582: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21882, t21891, t21901, t21905, t21906, t21917) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2071(t30, t21881, t508, t1518, t5517, t13584, t9375, t6785, t9335, t3833, t5824, t18280, t2255, t513, t5549, t605, zeta_threshold);
        let (t21918, t21931) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2072(t33, t6792, t9350, t3841, t6416, t1113, t20256, t2255, t516, t5557, t162, t21917, zeta_threshold);
        let (t21933, t21937, t21944, t21955) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2073(t30, t187, t21931, t1450, t6922, t6785, t9605, t3874, t5824, t1344, t18280, t2255, t5574, t605, zeta_threshold);
        let (t21956, t21969) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2074(t33, t6792, t9617, t3881, t6416, t1113, t1348, t20256, t2255, t5582, t21955, zeta_threshold);
    (t21882, t21891, t21901, t21905, t21906, t21918, t21931, t21933, t21937, t21944, t21956, t21969)
}
