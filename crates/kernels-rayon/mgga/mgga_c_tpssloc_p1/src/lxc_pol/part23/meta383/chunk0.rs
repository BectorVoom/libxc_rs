//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1185/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1185(t15418: f64, t1714: f64, t1174: f64, t1716: f64, t2402: f64, t15394: f64, t11554: f64, t1706: f64, t1709: f64, t44633: f64, t10401: f64, t15567: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52059 = t15418 * t1714;
    let t52081 = t1174 * t2402 * t1716;
    let t52100 = t15394 * t1714;
    let t52124 = t1706 * t11554;
    let t52281 = t1174 * t44633 * t1709;
    let t52627 = t15567 * t10401;
    (t52059, t52081, t52100, t52124, t52281, t52627)
}
