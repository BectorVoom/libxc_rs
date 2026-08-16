//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1428;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1429;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1430;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta270(t3121: f64, t884: f64, t3071: f64, t1023: f64, t2780: f64, t3036: f64, t67: f64, t3067: f64, t3186: f64, t3132: f64, t3062: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10393, t10394, t10397, t10398, t10401, t10402) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1428(t3121, t884, t3071, t1023, t2780, t3036, t67, t3067);
        let t10403 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1429(t10402, t3186);
        let (t10404, t10405, t10408) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1430(t3132, t884, t3071, t3062, t820);
    (t10393, t10394, t10397, t10398, t10401, t10402, t10403, t10404, t10405, t10408)
}
