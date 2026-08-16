//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2921/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2921(t17948: f64, t2940: f64, t17564: f64, t2933: f64, t959: f64, t17934: f64, t2952: f64, t1589: f64, t48766: f64, t14473: f64, t4493: f64, t18169: f64, t3216: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60857 = 0.20779030926817756511e3_f64 * t2940 * t17948;
    let t60860 = 0.6233709278045326953e3_f64 * t959 * t17564 * t2933;
    let t60862 = 0.17315859105681463759e2_f64 * t17934 * t2952;
    let t60864 = 0.11696447245269292414e1_f64 * t48766 * t1589;
    let t60866 = 0.23392894490538584828e1_f64 * t14473 * t4493;
    let t60867 = t18169 * t3216;
    (t60857, t60860, t60862, t60864, t60866, t60867)
}
