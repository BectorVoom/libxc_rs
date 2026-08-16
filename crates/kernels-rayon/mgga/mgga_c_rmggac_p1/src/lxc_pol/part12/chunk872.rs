//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 872/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk872(t39171: f64, t7720: f64, t236: f64, t495: f64, t7230: f64, t7248: f64, t9182: f64, t2144: f64, t3351: f64, t3352: f64, t5263: f64, t1596: f64, t1986: f64) -> (f64, f64, f64, f64) {
    let t39172 = t7720 * t39171;
    let t39177 = t7230 * t7248 * t236 * t9182 * t495;
    let t39181 = t3351 * t3352 * t2144 * t5263;
    let t39183 = t1986 * t1596;
    (t39172, t39177, t39181, t39183)
}
