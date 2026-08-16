//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 876/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk876(t7720: f64, t8587: f64, t34847: f64, t9206: f64, t1001: f64, t236: f64, t615: f64, t7230: f64, t9210: f64, t1166: f64, t1979: f64, t1982: f64, t2313: f64) -> (f64, f64, f64, f64) {
    let t39463 = t7720 * t8587;
    let t39465 = t34847 * t9206;
    let t39470 = t7230 * t9210 * t236 * t615 * t1001;
    let t39474 = t2313 * t1166 * t1979 * t1982;
    (t39463, t39465, t39470, t39474)
}
