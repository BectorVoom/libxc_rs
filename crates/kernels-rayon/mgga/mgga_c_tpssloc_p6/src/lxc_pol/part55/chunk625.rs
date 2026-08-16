//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 625/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk625(t3598: f64, t5059: f64, t1243: f64, t5000: f64, t1215: f64, t3612: f64, t1755: f64, t1235: f64, t1734: f64, t1246: f64, t491: f64, t5011: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5060 = t3598 * t5059;
    let t5064 = t5000 * t1243;
    let t5068 = t3612 * t1215;
    let t5069 = t1755 * t5068;
    let t5072 = t1235 * t1734;
    let t5073 = t5072 * t1246;
    let t5075 = t491 * t5011;
    (t5060, t5064, t5068, t5069, t5072, t5073, t5075)
}
