//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta166 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1048;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1049;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1050;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1051;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1052;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta166(t3: f64, t3931: f64, t112: f64, t1395: f64, t111: f64, t576: f64, t1401: f64, t2319: f64, t2363: f64, t577: f64, t671: f64, t2218: f64, t2221: f64, t2225: f64, t2232: f64, t1406: f64, t604: f64, t1437: f64, t645: f64, t1409: f64, t607: f64, t25: f64, t28: f64, t65: f64, t2219: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3932, t3938) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1048(t3, t3931, t112, t1395);
        let t3941 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1049(t111, t576);
        let (t3946, t3951) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1050(t1401, t2319, t2363, t3931, t3938, t3941, t577, t671, t2218, t2221, t2225, t2232);
        let (t3953, t3958, t3961) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1051(t1406, t604, t1437, t645, t1409, t607);
        let (t3962, t3966) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1052(t25, t28, t3961, t65, t2219, zeta_threshold);
    (t3932, t3938, t3941, t3946, t3951, t3953, t3958, t3961, t3962, t3966)
}
