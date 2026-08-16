//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 847/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk847(t815: f64, t836: f64, t812: f64, t2649: f64, t2617: f64, t2642: f64, t1891: f64, t67: f64, t246: f64, t232: f64, t2379: f64, t2646: f64) -> (f64, f64, f64) {
    let t9637 = t815 * t836;
    let t9638 = t812 * t9637;
    let t9639 = t9638 * t2649;
    let t9642 = t2617 * t2642;
    let t9645 = t1891 * t67;
    let t9646 = t9645 * t246;
    let t9647 = t232 * t2379;
    let t9649 = t9646 * t2646 * t9647;
    (t9639, t9642, t9649)
}
