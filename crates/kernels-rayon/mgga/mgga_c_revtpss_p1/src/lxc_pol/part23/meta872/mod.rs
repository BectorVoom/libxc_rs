//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta872 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2772;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2773;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta872(t22026: f64, t46929: f64, t808: f64, t22135: f64, t9744: f64, t1413: f64, t21969: f64, t547: f64, t807: f64, t221: f64, t22274: f64, t3978: f64, t46716: f64, t22279: f64, t9921: f64, t22255: f64, t3930: f64, t22259: f64, t9976: f64, t22125: f64, t2713: f64, t3964: f64, t13848: f64, t22096: f64, t9816: f64, t9818: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74362, t74364, t74402, t74421) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2772(t22026, t46929, t808, t22135, t9744, t1413, t21969, t547, t807, t221, t22274, t3978, t46716);
        let (t74425, t74427, t74429, t74437, t74461) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2773(t221, t22279, t3978, t9921, t22255, t3930, t22259, t9976, t22125, t2713, t3964, t13848, t22096, t9816, t9818);
    (t74362, t74364, t74402, t74421, t74425, t74427, t74429, t74437, t74461)
}
