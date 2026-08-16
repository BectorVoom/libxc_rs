//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta201 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk869;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta201(t10309: f64, t2826: f64, t136: f64, t10195: f64, t2770: f64, t9288: f64, t908: f64, t10250: f64, t883: f64, t9258: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10310, t10311, t10313, t10314, t10316) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk869(t10309, t2826, t136, t10195, t2770, t9288);
        let (t10317, t10318, t10319, t10320, t10321) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk870(t10316, t908, t136, t10250, t883, t9258);
    (t10310, t10311, t10313, t10314, t10316, t10317, t10318, t10319, t10320, t10321)
}
