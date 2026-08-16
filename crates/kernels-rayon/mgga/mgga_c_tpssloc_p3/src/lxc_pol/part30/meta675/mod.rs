//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta675 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2104;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2105;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta675(t91548: f64, t2022: f64, t6483: f64, t671: f64, t28821: f64, t6997: f64, t1441: f64, t4072: f64, t1874: f64, t28002: f64, t6525: f64, t7450: f64, t19440: f64, t71: f64, t33: f64, t55880: f64, t5441: f64, t645: f64, t72: f64, t5389: f64, t641: f64, t12568: f64, t1410: f64, t1860: f64, t1863: f64, t1865: f64, t22544: f64, t26084: f64, t26090: f64, t27950: f64, t27953: f64, t27956: f64, t27957: f64, t27961: f64, t6490: f64, t6495: f64, t6505: f64, t83741: f64, t83827: f64, t27960: f64, t4021: f64, t7431: f64, t1864: f64, t26009: f64, t26013: f64, t26016: f64, t27937: f64, t33567: f64, t6506: f64, t6510: f64, t83717: f64, t83830: f64, t90087: f64, t90091: f64, t90095: f64, t90098: f64, t90101: f64, t90104: f64, t9239: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93906, t96348, t96351, t96355, t96356, t96358, t96360, t96361) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2104(t91548, t2022, t6483, t671, t28821, t6997, t1441, t4072, t1874, t28002, t6525, t7450);
        let t96409 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2105(t19440, t71, t33, t55880, t5441, t645, t72, t5389, t641, t12568, t1410, t1860, t1863, t1865, t22544, t26084, t26090, t27950, t27953, t27956, t27957, t27961, t6490, t6495, t6505, t83741, t83827);
        let t96441 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2106(t27960, t645, t72, t4021, t7431, t1864, t5389, t1863, t22544, t26009, t26013, t26016, t27937, t33567, t6506, t6510, t83717, t83830, t90087, t90091, t90095, t90098, t90101, t90104, t9239);
    (t93906, t96348, t96351, t96355, t96356, t96358, t96360, t96361, t96409, t96441)
}
