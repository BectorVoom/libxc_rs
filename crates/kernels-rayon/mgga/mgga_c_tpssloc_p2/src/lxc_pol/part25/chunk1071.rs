//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1071/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1071(t1878: f64, t23033: f64, t253: f64, t254: f64, t234: f64, t6604: f64, t10143: f64, t25: f64, t28: f64, t870: f64, t1982: f64, t8944: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25154 = t1878 * t23033;
    let t25168 = t253 * t254;
    let t25248 = t6604 * t234;
    let t25373 = t10143 * t25;
    let t25891 = t870 * t28;
    let t25927 = t10143 * t28;
    let t26161 = t1982 * t8944;
    (t25154, t25168, t25248, t25373, t25891, t25927, t26161)
}
