//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1622;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1623;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta326(t124: f64, t1882: f64, t13847: f64, t5675: f64, t13845: f64, t5609: f64, t9794: f64, t9793: f64, t221: f64, t5627: f64, t9921: f64, t3978: f64, t2619: f64, t5635: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t13848 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1622(t124, t1882);
        let (t13850, t13851, t13858, t13878, t13880, t13887) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1623(t13847, t13848, t5675, t13845, t5609, t9794, t9793, t221, t5627, t9921, t3978, t2619, t5635);
    (t13848, t13850, t13851, t13858, t13878, t13880, t13887)
}
