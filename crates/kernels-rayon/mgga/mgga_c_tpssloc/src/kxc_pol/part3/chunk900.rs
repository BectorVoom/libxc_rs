//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 900/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk900(t761: f64, t9905: f64, t2250: f64, t751: f64, t707: f64, t2447: f64, t706: f64, t2509: f64, t746: f64, t9490: f64, t2531: f64, t2535: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9907 = 0.35089341735807877242e1_f64 * t761 * t9905;
    let t9909 = t751 * t2250;
    let t9910 = t707 * t9909;
    let t9912 = t706 * t2447;
    let t9919 = t2509 * t9490 * t746;
    let t9921 = 0.35089341735807877242e1_f64 * t761 * t9919;
    let t9922 = t2531 * t2535;
    (t9907, t9910, t9912, t9919, t9921, t9922)
}
