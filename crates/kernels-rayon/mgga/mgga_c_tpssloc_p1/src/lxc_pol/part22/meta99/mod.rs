//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk674;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk675;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk676;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk677;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk678;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk679;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk680;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta99(t2617: f64, t816: f64, t809: f64, t838: f64, t842: f64, t233: f64, t813: f64, t236: f64, t240: f64, t812: f64, t232: f64, t815: f64, t835: f64, t831: f64, t242: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2618, t2621, t2623) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk674(t2617, t816, t809, t838, t842);
        let t2627 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk675(t233, t813);
        let t2628 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk676(t236, t2627);
        let (t2629, t2630, t2632) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk677(t240, t2628, t812, t232);
        let t2638 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk678(t815, t835);
        let t2639 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk679(t2638, t812);
        let (t2640, t2642) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk680(t2639, t831, t242, t815);
    (t2618, t2621, t2623, t2627, t2628, t2629, t2630, t2632, t2638, t2639, t2640, t2642)
}
