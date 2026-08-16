//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1651/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1651(t43813: f64, t43816: f64, t43808: f64, t43810: f64, t43823: f64, t43826: f64, t43828: f64, t43830: f64, t43832: f64, t43837: f64, t43841: f64, t43845: f64, t43849: f64, t43854: f64) -> f64 {
    let t45106 = 0.5356037037037037037e1_f64 * t43813;
    let t45107 = 0.16979925925925925926e1_f64 * t43816;
    let t45118 = -0.94674375e0_f64 * t43808 + 0.1262325e1_f64 * t43810 + t45106 + t45107 - 0.6618234375e1_f64 * t43823 - 0.52945875e1_f64 * t43826 - 0.166712e1_f64 * t43828 - 0.41318e1_f64 * t43830 + 0.13772666666666666667e1_f64 * t43832 + 0.34431666666666666667e1_f64 * t43837 - 0.13772666666666666667e1_f64 * t43841 + 0.185931e2_f64 * t43845 + 0.41318e1_f64 * t43849 - 0.123954e2_f64 * t43854;
    t45118
}
