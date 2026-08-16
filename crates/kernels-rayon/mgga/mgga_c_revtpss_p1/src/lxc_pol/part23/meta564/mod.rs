//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta564 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2136;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta564(t117: f64, t22746: f64, t1312: f64, t1518: f64, t18245: f64, t22633: f64, t22639: f64, t4248: f64, t5920: f64, t7889: f64, t13584: f64, t22186: f64, t22188: f64, t22191: f64, t22196: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22747, t22758, t22762, t22763) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2136(t117, t22746, t1312, t1518, t18245, t22633, t22639, t4248, t5920, t7889, t13584, t22186);
        let (t22764, t22765, t22766, t22767) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2137(t22188, t22191, t22196, t22762, t22763, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
    (t22747, t22758, t22762, t22763, t22764, t22765, t22766, t22767)
}
