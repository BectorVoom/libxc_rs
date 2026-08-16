//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 680/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk680(t1291: f64, t2205: f64, t7746: f64, t7750: f64, t7752: f64, t7756: f64, t7758: f64, t7760: f64, t7762: f64, t7764: f64) -> (f64, f64) {
    let t7812 = t2205 * t1291;
    let t7823 = 0.9375e-1_f64 * t7746 - 0.9375e-1_f64 * t7750 - 0.25e0_f64 * t7752 + 0.625e-1_f64 * t7756 - 0.20234375e-1_f64 * t7758 + 0.20234375e-1_f64 * t7760 + 0.10791666666666666667e0_f64 * t7762 - 0.26979166666666666667e-1_f64 * t7764;
    (t7812, t7823)
}
