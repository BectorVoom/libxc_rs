//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1336/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1336(t1891: f64, t67: f64, t246: f64, t232: f64, t2379: f64, t2646: f64, t2645: f64, t2647: f64, t9626: f64, t210: f64, t2553: f64, t804: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9645 = t1891 * t67;
    let t9646 = t9645 * t246;
    let t9647 = t232 * t2379;
    let t9649 = t9646 * t2646 * t9647;
    let t9653 = t2645 * t9626 * t2647;
    let t9657 = t210 * t804 * t2553;
    (t9645, t9646, t9647, t9649, t9653, t9657)
}
