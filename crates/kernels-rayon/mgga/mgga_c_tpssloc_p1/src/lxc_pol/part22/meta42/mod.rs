//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta42 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk301;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk302;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk303;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk304;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk305;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk306;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk307;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk308;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk309;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta42(t244: f64, t248: f64, t836: f64, t238: f64, t234: f64, t236: f64, t240: f64, t812: f64, t200: f64, t243: f64, t241: f64, t67: f64, t776: f64, t820: f64, t249: f64, t787: f64, t803: f64, t805: f64, t809: f64, t817: f64, t831: f64, t218: f64, t225: f64, t253: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t838 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk301(t244, t248, t836);
        let (t840, t841) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk302(t238, t838, t234, t236);
        let t842 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk303(t240, t841);
        let t843 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk304(t812, t842);
        let t845 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk305(t200, t243);
        let t847 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk306(t241, t67, t845);
        let t849 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk307(t776, t820, t847);
        let t852 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk308(t249, t787, t803, t805, t809, t817, t831, t840, t843, t849);
        let (t853, t855) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk309(t218, t852, t225, t253);
    (t838, t840, t841, t842, t843, t845, t847, t849, t852, t853, t855)
}
