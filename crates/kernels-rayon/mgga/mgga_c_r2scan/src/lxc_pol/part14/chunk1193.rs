//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1193/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1193(t3275: f64, t3472: f64, t39286: f64, t41196: f64, t41199: f64, t41201: f64, t41205: f64, t41208: f64, t41211: f64, t41213: f64, t41216: f64, t41219: f64, t41221: f64, t41223: f64, t41225: f64, t41227: f64, t41230: f64, t41233: f64) -> (f64, f64) {
    let t41236 = 5.0_f64 / 16.0_f64 * t3275 * t3472 * t39286;
    let t41237 = t41196 + t41199 + t41201 + t41205 + t41208 - t41211 + t41213 - t41216 - t41219 + t41221 + t41223 - t41225 - t41227 + t41230 + t41233 + t41236;
    (t41236, t41237)
}
