//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 949/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk949(t191: f64, t192: f64, t28020: f64, t33153: f64, t33151: f64, t33106: f64, t7440: f64, t8513: f64, t119942: f64, t1433: f64, t119878: f64, t1409: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126022 = t28020 * t191 * t192;
    let t126035 = 4.0_f64 * t33153;
    let t126036 = 4.0_f64 * t33151;
    let t126046 = t8513 * t33106 * t7440;
    let t126062 = t8513 * t119942 * t1433;
    let t126065 = t119878 * t1409;
    (t126022, t126035, t126036, t126046, t126062, t126065)
}
