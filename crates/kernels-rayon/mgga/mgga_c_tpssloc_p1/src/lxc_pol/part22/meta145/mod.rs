//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta145 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk932;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta145(t4370: f64, t894: f64, t1547: f64, t2815: f64, t896: f64, t901: f64, t1553: f64, t699: f64, t2826: f64, t4338: f64, t136: f64, t4343: f64, t908: f64, t4347: f64, t2766: f64, t2810: f64, t2823: f64, t2824: f64, t4335: f64, t4340: f64, t4345: f64, t4349: f64, t4363: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4371, t4378, t4379, t4381, t4384) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk932(t4370, t894, t1547, t2815, t896, t901, t1553, t699);
        let (t4386, t4387, t4389, t4390, t4392, t4393, t4395) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk933(t2826, t4338, t136, t4343, t908, t4347, t2766, t2810, t2823, t2824, t4335, t4340, t4345, t4349, t4363, t4371, t4379, t4381, t4384);
    (t4371, t4378, t4379, t4381, t4384, t4386, t4387, t4389, t4390, t4392, t4393, t4395)
}
