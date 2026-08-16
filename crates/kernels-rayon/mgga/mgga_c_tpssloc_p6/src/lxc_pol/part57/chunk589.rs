//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 589/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk589(t1933: f64, t7573: f64, t1597: f64, t343: f64, t6734: f64, t1615: f64, t68: f64, t360: f64, t6744: f64, t1611: f64, t1941: f64, t1607: f64, t1618: f64, t1622: f64, t1935: f64, t1937: f64, t378: f64, t6716: f64, t6717: f64, t6728: f64, t6742: f64, t6755: f64, t6763: f64, t6765: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7574 = t1933 * t7573;
    let t7577 = t1597 * t343;
    let t7578 = t7577 * t6734;
    let t7581 = t1615 * t68;
    let t7582 = t7581 * t360;
    let t7583 = t6744 * t7582;
    let t7586 = t1611 * t1941;
    let t7593 = t6716 + t6717 * t1607 / 288.0_f64 + t6728 + 0.10093189023535097714e-3_f64 * t7574 * t1937 - 0.10093189023535097714e-3_f64 * t1935 * t7578 + 0.10093189023535097714e-3_f64 * t6742 * t7583 + t7586 * t378 / 1536.0_f64 + t6755 * t1618 / 1536.0_f64 + t6763 + t6765 * t1622 / 2304.0_f64;
    (t7574, t7577, t7578, t7582, t7583, t7586, t7593)
}
