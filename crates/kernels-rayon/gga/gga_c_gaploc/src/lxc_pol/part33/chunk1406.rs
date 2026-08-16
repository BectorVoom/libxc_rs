//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1406/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1406(t12125: f64, t1580: f64, t188: f64, t189: f64, t193: f64, t31412: f64, t31414: f64, t31416: f64, t35201: f64, t35206: f64, t35209: f64, t35211: f64, t35214: f64, t35219: f64, t35226: f64, t35229: f64, t35232: f64, t3695: f64, t3715: f64, t38313: f64, t4585: f64, t4637: f64, t557: f64) -> f64 {
    let t38863 = -t35201 - t35206 - t35209 + t35211 - t35214 - t35219 - t31412 - t31414 - 0.76685851907841499354e0_f64 * t31416 + 0.79445533226334281487e-1_f64 * t557 * t4585 * t3695 + 0.23005755572352449806e1_f64 * t4637 * t3715 + 0.46011511144704899612e1_f64 * t1580 * t12125 + 0.35750489951850426669e0_f64 * t188 * t189 * t38313 * t193 - t35226 - t35229 + t35232;
    t38863
}
