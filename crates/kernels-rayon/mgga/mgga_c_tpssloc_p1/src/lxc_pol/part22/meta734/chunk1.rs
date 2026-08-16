//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2411/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2411(t68785: f64, t68798: f64, t68812: f64, t68825: f64, t68839: f64, t68851: f64, t68864: f64, t68877: f64, t893: f64, t913: f64, t21303: f64, t42023: f64) -> (f64, f64) {
    let t68883 = 1.0_f64 * t893 * (t68785 + t68798 + t68812 + t68825 + t68839 + t68851 + t68864 + t68877) * t913;
    let t68885 = 0.51726012919273400301e3_f64 * t42023 * t21303;
    (t68883, t68885)
}
