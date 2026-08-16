//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1229/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1229(t10497: f64, t2183: f64, t11068: f64, t27002: f64, t7788: f64, t11178: f64, t1250: f64, t251: f64, t11061: f64, t7790: f64, t27013: f64, t3489: f64) -> (f64, f64, f64, f64, f64) {
    let t92581 = t2183 * t10497;
    let t92587 = t7788 * t11068 * t27002;
    let t92590 = t11178 * t251 * t1250;
    let t92600 = t7788 * t11061 * t7790;
    let t92604 = t27013 * t3489;
    (t92581, t92587, t92590, t92600, t92604)
}
