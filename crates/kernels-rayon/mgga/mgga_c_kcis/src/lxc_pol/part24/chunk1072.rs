//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1072/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1072(t26933: f64, t8069: f64, t1196: f64, t4999: f64, t26924: f64, t8072: f64, t283: f64, t5082: f64) -> (f64, f64, f64, f64) {
    let t28053 = t26933 * t8069;
    let t28055 = t4999 * t1196;
    let t28057 = t26924 * t8072;
    let t28059 = t5082 * t283;
    (t28053, t28055, t28057, t28059)
}
