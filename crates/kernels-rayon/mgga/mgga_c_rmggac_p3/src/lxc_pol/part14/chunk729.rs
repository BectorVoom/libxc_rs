//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 729/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk729(t34750: f64, t34755: f64, t388: f64, t140: f64, t673: f64, t465: f64, t7472: f64) -> (f64, f64, f64, f64) {
    let t34757 = t34755 * t388 * t34750;
    let t34759 = t673 * t140;
    let t34760 = t465 * t34759;
    let t34761 = t7472 * t34760;
    (t34757, t34759, t34760, t34761)
}
