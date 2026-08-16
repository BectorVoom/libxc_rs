//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta143 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk925;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk926;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk927;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk928;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta143(t1409: f64, t2770: f64, t607: f64, t2768: f64, t123: f64, t2775: f64, t882: f64, t3966: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4337 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk925(t1409, t2770);
        let t4338 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk926(t4337, t607);
        let (t4339, t4340, t4342) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk927(t2768, t4338, t123, t1409, t2775);
        let t4343 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk928(t4342, t607);
        let (t4344, t4345, t4347) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk929(t4343, t882, t123, t3966, t883);
    (t4337, t4338, t4339, t4340, t4342, t4343, t4344, t4345, t4347)
}
