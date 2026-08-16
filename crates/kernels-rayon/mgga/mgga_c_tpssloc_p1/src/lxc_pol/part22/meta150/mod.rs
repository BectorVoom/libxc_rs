//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta150 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk950;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk951;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk952;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk953;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta150(t4343: f64, t4518: f64, t3966: f64, t978: f64, t977: f64, t135: f64, t1599: f64, t973: f64, t1597: f64, t2987: f64, t2990: f64, t2824: f64, t3003: f64, t4384: f64, t4387: f64, t4390: f64, t4393: f64, t340: f64, t343: f64, t974: f64, t984: f64, t1593: f64, t1600: f64, t2958: f64, t2960: f64, t2969: f64, t2972: f64, t2975: f64, t2986: f64, t4507: f64, t4511: f64, t4515: f64, t381: f64, t1049: f64, t1603: f64, t1604: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4519, t4522, t4523, t4528, t4529, t4531) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk950(t4343, t4518, t3966, t978, t977, t135, t1599, t973, t1597, t2987);
        let (t4532, t4540) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk951(t2990, t4531, t2824, t3003, t4384, t4387, t4390, t4393);
        let (t4542, t4543, t4546) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk952(t340, t4540, t343, t974);
        let (t4548, t4552) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk953(t1597, t984, t343, t4546, t1593, t1600, t2958, t2960, t2969, t2972, t2975, t2986, t4507, t4511, t4515, t4519, t4523, t4529, t4532, t4543, t973);
        let (t4553, t4555, t4557) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk954(t381, t4552, t1049, t1603, t1604, t225);
    (t4522, t4528, t4529, t4531, t4540, t4542, t4546, t4548, t4552, t4553, t4555, t4557)
}
