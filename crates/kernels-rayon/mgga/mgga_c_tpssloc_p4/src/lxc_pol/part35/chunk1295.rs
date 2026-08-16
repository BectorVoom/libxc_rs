//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1295/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1295(t2122: f64, t8034: f64, t8003: f64, t85660: f64, t8015: f64, t1751: f64, t24594: f64, t8074: f64, t85917: f64, t1089: f64, t7327: f64, t131: f64, t1419: f64, t23598: f64, t467: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94514 = t8034 * t2122;
    let t94525 = t85660 * t8003;
    let t94701 = t85660 * t8015;
    let t94754 = t24594 * t1751;
    let t94784 = t85917 * t8074;
    let t94837 = t7327 * t1751 * t1089;
    let t94858 = t1419 * t23598 * t131 * t467;
    (t94514, t94525, t94701, t94754, t94784, t94837, t94858)
}
