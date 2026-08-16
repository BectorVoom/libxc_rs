//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1834;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta562(t1408: f64, t2745: f64, t25365: f64, t81547: f64, t1530: f64, t2553: f64, t22960: f64, t12971: f64, t25: f64, t2379: f64, t4255: f64, t606: f64, t870: f64, t25213: f64, t6547: f64, t22986: f64, t23270: f64, t25053: f64, t4119: f64, t857: f64, t865: f64, t4300: f64, t776: f64, t1888: f64, t2717: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86806, t86810, t86815, t86816, t86821, t86825, t86830) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1834(t1408, t2745, t25365, t81547, t1530, t2553, t22960, t12971, t25, t2379, t4255, t606, t870);
        let (t86843, t86847, t86852, t86857, t86862) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1835(t25213, t6547, t22986, t23270, t25053, t2553, t4119, t857, t865, t4300, t776, t1888, t2717);
    (t86806, t86810, t86815, t86816, t86821, t86825, t86830, t86843, t86847, t86852, t86857, t86862)
}
