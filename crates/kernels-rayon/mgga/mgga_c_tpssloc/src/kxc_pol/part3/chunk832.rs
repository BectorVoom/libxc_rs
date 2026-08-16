//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 832/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk832(t1755: f64, t5068: f64, t1235: f64, t1734: f64, t1246: f64, t491: f64, t5011: f64, t1215: f64, t1932: f64, t475: f64, t1751: f64, t493: f64, t5052: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5069 = t1755 * t5068;
    let t5072 = t1235 * t1734;
    let t5073 = t5072 * t1246;
    let t5075 = t491 * t5011;
    let t5076 = t5075 * t1246;
    let t5079 = t1932 * t1215 * t475;
    let t5080 = t1755 * t5079;
    let t5083 = t1751 * t1215;
    let t5084 = t5083 * t1246;
    let t5086 = t493 * t5052;
    (t5069, t5072, t5073, t5075, t5076, t5079, t5080, t5084, t5086)
}
