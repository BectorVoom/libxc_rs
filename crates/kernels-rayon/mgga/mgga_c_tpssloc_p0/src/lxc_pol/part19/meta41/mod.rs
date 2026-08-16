//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta41 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk285;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk286;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk287;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk288;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk289;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk290;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta41(t819: f64, t820: f64, t829: f64, t590: f64, t61: f64, t241: f64, t244: f64, t248: f64, t238: f64, t234: f64, t236: f64, t240: f64, t812: f64, t200: f64, t243: f64, t67: f64, t776: f64, t249: f64, t787: f64, t803: f64, t805: f64, t809: f64, t817: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t831 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk285(t819, t820, t829);
        let t835 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk286(t590, t61);
        let t836 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk287(t241, t835);
        let (t838, t840, t841, t842, t843) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk288(t244, t248, t836, t238, t234, t236, t240, t812);
        let t845 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk289(t200, t243);
        let (t847, t849) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk290(t241, t67, t845, t776, t820);
        let t852 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk291(t249, t787, t803, t805, t809, t817, t831, t840, t843, t849);
    (t831, t835, t836, t838, t841, t842, t843, t845, t847, t849, t852)
}
