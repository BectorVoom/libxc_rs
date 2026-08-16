//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 590/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk590(t349: f64, t7593: f64, t1634: f64, t1955: f64, t3174: f64, t1539: f64, t6785: f64, t6784: f64, t1599: f64, t1949: f64, t1629: f64, t6800: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7594 = t349 * t7593;
    let t7599 = t1955 * t1634;
    let t7600 = t3174 * t7599;
    let t7603 = t6785 * t1539;
    let t7604 = t6784 * t7603;
    let t7607 = t1599 * t1949;
    let t7610 = t1629 * t6800;
    (t7594, t7600, t7603, t7604, t7607, t7610)
}
