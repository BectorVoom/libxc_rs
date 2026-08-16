//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta657 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2183;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta657(t26135: f64, t5113: f64, t1983: f64, t23857: f64, t7753: f64, t24991: f64, t6876: f64, t25992: f64, t22592: f64, t7685: f64, t22948: f64, t5161: f64, t1845: f64, t3914: f64, t26161: f64, t26162: f64, t24994: f64, t6875: f64, t24996: f64, t24995: f64, t34475: f64, t5308: f64, t26503: f64, t6999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90410, t90418, t90421, t90428, t90434, t90436) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2183(t26135, t5113, t1983, t23857, t7753, t24991, t6876, t25992, t22592, t7685, t22948, t5161);
        let (t90440, t90444, t90447, t90450) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2184(t1845, t3914, t26161, t26162, t24994, t6875, t24996, t24995, t34475, t5308, t1983, t26503, t6999);
    (t90410, t90418, t90421, t90428, t90434, t90436, t90440, t90444, t90447, t90450)
}
