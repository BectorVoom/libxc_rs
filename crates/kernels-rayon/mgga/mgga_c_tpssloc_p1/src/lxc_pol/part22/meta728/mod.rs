//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta728 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2382;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2383;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2384;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2385;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2386;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2387;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2388;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta728(t136: f64, t68554: f64, t908: f64, t43317: f64, t48140: f64, t68513: f64, t49200: f64, t59657: f64, t60163: f64, t60168: f64, t60173: f64, t68536: f64, t68541: f64, t68545: f64, t68549: f64, t68552: f64, t67060: f64, t883: f64, t123: f64, t882: f64, t68543: f64, t68547: f64, t68458: f64, t68534: f64, t2768: f64, t68539: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68556, t68563, t68565) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2382(t136, t68554, t908, t43317, t48140, t68513, t49200, t59657, t60163, t60168, t60173, t68536, t68541, t68545, t68549, t68552);
        let (t68569, t68571) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2383(t67060, t883, t123, t882);
        let t68577 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2384(t123, t68543, t882);
        let t68580 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2385(t123, t68547, t882);
        let t68583 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2386(t123, t68554, t882);
        let t68586 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2387(t123, t68458, t882);
        let t68589 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2388(t123, t68534, t882);
        let t68592 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2389(t123, t2768, t68539);
    (t68556, t68563, t68565, t68569, t68571, t68577, t68580, t68583, t68586, t68589, t68592)
}
