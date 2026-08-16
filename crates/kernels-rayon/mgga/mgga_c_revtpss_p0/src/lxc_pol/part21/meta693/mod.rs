//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta693 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2514;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2515;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta693(t12378: f64, t300: f64, t13062: f64, t13064: f64, t3172: f64, t1247: f64, t13075: f64, t1209: f64, t13126: f64, t17708: f64, t127: f64, t12988: f64, t12989: f64, t371: f64, t1203: f64, t12626: f64, t225: f64, t12967: f64, t12995: f64, t12627: f64, t1269: f64, t3566: f64, t3727: f64, t12640: f64, t44842: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45319, t45346, t45352, t45371, t45382) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2514(t12378, t300, t13062, t13064, t3172, t1247, t13075, t1209, t13126, t17708, t127, t12988, t12989, t371);
        let (t45384, t45385, t45389, t45427, t45430, t45433, t45438) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2515(t1203, t12626, t225, t12967, t12995, t12627, t1269, t3566, t3727, t12640, t44842, t487);
    (t45319, t45346, t45352, t45371, t45382, t45384, t45385, t45389, t45427, t45430, t45433, t45438)
}
