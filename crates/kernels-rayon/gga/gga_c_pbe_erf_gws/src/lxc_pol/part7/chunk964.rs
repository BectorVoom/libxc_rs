//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 964/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk964(t1748: f64, t1781: f64, t184: f64, t221: f64, t1735: f64, t5343: f64, t17807: f64, t17809: f64, t17813: f64, t17815: f64, t17818: f64, t17822: f64, t17825: f64, t17827: f64, t17832: f64) -> (f64, f64, f64) {
    let t17836 = 8.0_f64 / 5.0_f64 * t1781 * t1748 * t184 * t221;
    let t17838 = 8.0_f64 / 5.0_f64 * t5343 * t1735;
    let t17839 = -t17807 + t17809 + t17813 - t17815 + t17818 - t17822 - t17825 - t17827 + t17832 + t17836 + t17838;
    (t17836, t17838, t17839)
}
