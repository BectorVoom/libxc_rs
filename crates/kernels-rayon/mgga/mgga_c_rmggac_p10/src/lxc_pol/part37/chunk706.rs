//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 706/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk706(t2084: f64, t2123: f64, t2145: f64, t27: f64, t14088: f64, t21: f64, t132: f64, t14090: f64, t240: f64, t31: f64, t4738: f64, t71: f64) -> (f64, f64) {
    let t69689 = t2145 * t27 * t2084 * t2123;
    let t69695 = t21 * t14088;
    let t69701 = t69695 * t14090 * t71 * t132 * t240 * t4738 * t31;
    (t69689, t69701)
}
