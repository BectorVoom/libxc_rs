//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1212;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1213;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta299(t1041: f64, t10459: f64, t1008: f64, t349: f64, t1011: f64, t1013: f64, t363: f64, t3034: f64, t6793: f64, t368: f64, t3131: f64, t360: f64, t248: f64, t2776: f64, t3051: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10460, t10469, t10470, t10471) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1212(t1041, t10459, t1008, t349, t1011);
        let (t10472, t10474, t10477, t10478, t10480, t10482, t10489) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1213(t10470, t10471, t1013, t363, t3034, t6793, t368, t3131, t360, t248, t2776, t3051);
    (t10460, t10469, t10470, t10471, t10472, t10474, t10477, t10478, t10480, t10482, t10489)
}
