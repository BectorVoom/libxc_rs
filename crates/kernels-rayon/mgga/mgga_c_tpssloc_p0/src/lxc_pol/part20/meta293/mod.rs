//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta293 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1504;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1505;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1506;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1507;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta293(t3087: f64, t372: f64, t364: f64, t354: f64, t1009: f64, t3020: f64, t1011: f64, t1019: f64, t1040: f64, t3077: f64, t2775: f64, t283: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10956, t10957) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1504(t3087, t372, t364, t354);
        let (t10960, t10961, t10962) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1505(t1009, t3020, t1011, t1019);
        let t10965 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1506(t1040, t3077);
        let t10969 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1507(t2775, t283);
    (t10956, t10957, t10960, t10961, t10962, t10965, t10969)
}
