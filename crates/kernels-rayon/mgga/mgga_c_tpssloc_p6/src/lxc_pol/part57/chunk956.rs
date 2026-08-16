//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 956/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk956(t5612: f64, t6605: f64, t6612: f64, t23046: f64, t5585: f64, t1894: f64, t23078: f64, t5527: f64, t59: f64, t5624: f64, t8343: f64, t23097: f64, t28395: f64) -> (f64, f64, f64, f64, f64) {
    let t126325 = t6605 * t6612 * t5612;
    let t126328 = t6605 * t23046 * t5585;
    let t126332 = t23078 * t1894 * t59 * t5527;
    let t126334 = t8343 * t5624;
    let t126337 = t23097 * t6612 * t28395;
    (t126325, t126328, t126332, t126334, t126337)
}
