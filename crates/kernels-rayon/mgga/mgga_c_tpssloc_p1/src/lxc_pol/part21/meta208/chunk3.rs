//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1281/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1281(t1246: f64, t5072: f64, t491: f64, t5011: f64, t1215: f64, t1932: f64, t475: f64) -> (f64, f64, f64, f64) {
    let t5073 = t5072 * t1246;
    let t5075 = t491 * t5011;
    let t5076 = t5075 * t1246;
    let t5079 = t1932 * t1215 * t475;
    (t5073, t5075, t5076, t5079)
}
