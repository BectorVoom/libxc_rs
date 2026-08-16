//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1401/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1401(t75911: f64, t43791: f64, t75836: f64, t11219: f64, t136: f64, t43763: f64, t43761: f64, t3242: f64, t75847: f64, t3297: f64, t3247: f64, t1113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77953 = -t75911;
    let t77957 = t43791 * t75836;
    let t77959 = t136 * t11219 * t77957;
    let t77961 = t43763 * t75836;
    let t77963 = t136 * t43761 * t77961;
    let t77965 = t3242 * t75847;
    let t77967 = t136 * t3297 * t77965;
    let t77969 = t3247 * t75847;
    let t77971 = t136 * t1113 * t77969;
    (t77953, t77957, t77959, t77961, t77963, t77965, t77967, t77969, t77971)
}
