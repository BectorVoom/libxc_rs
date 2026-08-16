//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2618/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2618(t3566: f64, t5023: f64, t15734: f64, t3490: f64, t11789: f64, t1227: f64, t248: f64, t4733: f64, t11712: f64, t11913: f64, t491: f64, t11887: f64, t52834: f64) -> (f64, f64, f64, f64, f64) {
    let t53507 = t3566 * t5023;
    let t53515 = t3490 * t15734;
    let t53519 = t1227 * t248 * t11789 * t4733;
    let t53545 = t11712 * t11913 * t491;
    let t53565 = t52834 * t11887;
    (t53507, t53515, t53519, t53545, t53565)
}
