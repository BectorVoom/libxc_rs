//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1836;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta563(t1888: f64, t23270: f64, t25044: f64, t2742: f64, t23168: f64, t25342: f64, t25345: f64, t82038: f64, t1519: f64, t213: f64, t225: f64, t22986: f64, t23272: f64, t2379: f64, t25038: f64, t25053: f64, t25054: f64, t82159: f64, t25229: f64, t23222: f64, t25224: f64, t6552: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86866, t86868, t86870, t86875) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1836(t1888, t23270, t25044, t2742, t23168, t25342, t25345, t82038, t1519, t213, t225, t22986, t23272);
        let (t86881, t86884, t86886, t86891) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1837(t23270, t2379, t25038, t25053, t22986, t25054, t82159, t23168, t25229, t23222, t25224, t6552);
    (t86866, t86868, t86870, t86875, t86881, t86884, t86886, t86891)
}
