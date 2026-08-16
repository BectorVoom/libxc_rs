//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 847/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk847(t813: f64, t224: f64, t2827: f64, t2627: f64, t883: f64, t273: f64, t2787: f64, t286: f64, t791: f64, t709: f64, t804: f64, t36: f64, t7777: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11882 = t813 * t813;
    let t11883 = 1.0_f64 / t11882;
    let t11889 = t224 * t2827;
    let t11893 = t883 * t2627;
    let t11898 = 0.46785788981077169656e1_f64 * t286 * t791 * t2787 * t273;
    let t11900 = 120.0_f64 * t709 * t804;
    let t11906 = 840.0_f64 * t36 * t7777 * t88;
    (t11883, t11889, t11893, t11898, t11900, t11906)
}
