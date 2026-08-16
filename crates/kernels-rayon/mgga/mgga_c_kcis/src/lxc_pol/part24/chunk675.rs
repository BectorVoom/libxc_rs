//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 675/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk675(t2193: f64, t7784: f64, t1240: f64, t251: f64, t1250: f64) -> (f64, f64, f64) {
    let t7786 = 0.11584201388888888889e-3_f64 * t2193 * t7784;
    let t7787 = t1240 * t251;
    let t7788 = t7787 * t1250;
    (t7786, t7787, t7788)
}
