//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1231/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1231(t225: f64, t497: f64, t6238: f64, t462: f64, t27751: f64, t8014: f64, t1887: f64, t29584: f64) -> (f64, f64, f64, f64) {
    let t29670 = t6238 * t225 * t497;
    let t29671 = t462 * t29670;
    let t29674 = t27751 * t8014;
    let t29678 = t29584 * t1887;
    (t29670, t29671, t29674, t29678)
}
