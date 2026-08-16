//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1430/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1430(t28563: f64, t28566: f64, t28569: f64, t33257: f64, t33259: f64, t33261: f64, t33269: f64, t33271: f64, t33274: f64, t33282: f64, t33284: f64, t33292: f64, t33297: f64, t33299: f64, t33311: f64, t33313: f64) -> f64 {
    let t39106 = -t33257 - t33259 + t33261 + t33269 + t33271 + t33274 + 0.76685851907841499354e0_f64 * t28563 + 0.76685851907841499354e0_f64 * t28566 + 0.38342925953920749677e0_f64 * t28569 - t33282 - t33284 - t33292 - t33297 - t33299 + t33311 + t33313;
    t39106
}
