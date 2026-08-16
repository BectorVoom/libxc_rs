//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1041/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1041(t26836: f64, t15573: f64, t7774: f64, t7772: f64, t11151: f64, t251: f64, t1250: f64) -> (f64, f64, f64, f64, f64) {
    let t27053 = 0.38691203703703703703e-3_f64 * t26836;
    let t27055 = t15573 * t7774;
    let t27056 = t7772 * t27055;
    let t27069 = t11151 * t251;
    let t27070 = t27069 * t1250;
    (t27053, t27055, t27056, t27069, t27070)
}
