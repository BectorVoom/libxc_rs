//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1840;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta565(t1888: f64, t25045: f64, t82159: f64, t6562: f64, t7488: f64, t82133: f64, t25225: f64, t6547: f64, t23168: f64, t25338: f64, t23012: f64, t7485: f64, t23270: f64, t2719: f64, t46488: f64, t25046: f64, t6579: f64, t1484: f64, t2717: f64, t22986: f64, t7489: f64, t13460: f64, t1880: f64, t6553: f64, t6571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86933, t86940, t86942, t86950, t86955) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1840(t1888, t25045, t82159, t6562, t7488, t82133, t25225, t6547, t23168, t25338, t23012, t7485);
        let (t86961, t86967, t86972, t86991, t86997) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1841(t1888, t23270, t2719, t46488, t25046, t6579, t1484, t2717, t22986, t23012, t7489, t13460, t1880, t6553, t6571);
    (t86933, t86940, t86942, t86950, t86955, t86961, t86967, t86972, t86991, t86997)
}
