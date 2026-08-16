//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1379/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1379(t12063: f64, t1359: f64, t1424: f64, t34143: f64, t34145: f64, t34148: f64, t34151: f64, t34153: f64, t34156: f64, t34178: f64, t34181: f64, t34186: f64, t34189: f64, t34191: f64, t34216: f64, t34220: f64, t34242: f64, t34245: f64, t544: f64) -> f64 {
    let t38481 = -t34143 - t34145 - t34148 - t34151 - t34153 - t34156 + t34178 - t34181 - t34186 - t34189 - t34191 - t34216 - t34220 - 0.79445533226334281486e-1_f64 * t544 * t1359 * t12063 * t1424 - t34242 + t34245;
    t38481
}
