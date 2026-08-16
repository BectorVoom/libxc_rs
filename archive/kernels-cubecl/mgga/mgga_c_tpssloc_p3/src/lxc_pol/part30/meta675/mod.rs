//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta675 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2104;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2105;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta675<F: Float>(t91548: F, t2022: F, t6483: F, t671: F, t28821: F, t6997: F, t1441: F, t4072: F, t1874: F, t28002: F, t6525: F, t7450: F, t19440: F, t71: F, t33: F, t55880: F, t5441: F, t645: F, t72: F, t5389: F, t641: F, t12568: F, t1410: F, t1860: F, t1863: F, t1865: F, t22544: F, t26084: F, t26090: F, t27950: F, t27953: F, t27956: F, t27957: F, t27961: F, t6490: F, t6495: F, t6505: F, t83741: F, t83827: F, t27960: F, t4021: F, t7431: F, t1864: F, t26009: F, t26013: F, t26016: F, t27937: F, t33567: F, t6506: F, t6510: F, t83717: F, t83830: F, t90087: F, t90091: F, t90095: F, t90098: F, t90101: F, t90104: F, t9239: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t93906, t96348, t96351, t96355, t96356, t96358, t96360, t96361) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2104::<F>(t91548, t2022, t6483, t671, t28821, t6997, t1441, t4072, t1874, t28002, t6525, t7450);
        let t96409 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2105::<F>(t19440, t71, t33, t55880, t5441, t645, t72, t5389, t641, t12568, t1410, t1860, t1863, t1865, t22544, t26084, t26090, t27950, t27953, t27956, t27957, t27961, t6490, t6495, t6505, t83741, t83827);
        let t96441 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2106::<F>(t27960, t645, t72, t4021, t7431, t1864, t5389, t1863, t22544, t26009, t26013, t26016, t27937, t33567, t6506, t6510, t83717, t83830, t90087, t90091, t90095, t90098, t90101, t90104, t9239);
    (t93906, t96348, t96351, t96355, t96356, t96358, t96360, t96361, t96409, t96441)
}
