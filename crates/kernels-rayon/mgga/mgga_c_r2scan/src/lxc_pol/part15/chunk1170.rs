//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1170/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1170(t11837: f64, t1584: f64, t26307: f64, t3308: f64, t574: f64, t3309: f64, t7566: f64, t10725: f64, t2651: f64, t37754: f64, t546: f64, t39841: f64, t6087: f64) -> (f64, f64, f64, f64, f64) {
    let t40024 = t1584 * t11837;
    let t40027 = t574 * t3308 * t26307;
    let t40029 = t7566 * t3309;
    let t40031 = t2651 * t10725;
    let t40033 = t546 * t37754;
    let t40035 = t40033 * t39841 * t6087;
    (t40024, t40027, t40029, t40031, t40035)
}
