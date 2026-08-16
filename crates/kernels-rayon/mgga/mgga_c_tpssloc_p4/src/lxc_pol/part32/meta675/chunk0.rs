//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2111/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2111(t24716: f64, t4997: f64, t15492: f64, t7339: f64, t15734: f64, t7345: f64, t25588: f64, t461: f64, t7324: f64, t1244: f64, t1742: f64, t3068: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t95542 = t24716 * t4997 / 1152.0_f64;
    let t95545 = t7339 * t15492 / 1152.0_f64;
    let t95550 = t7345 * t15734;
    let t95556 = t7324 * t25588 * t461;
    let t95566 = t1244 * sigma2 * t1742 * t3068;
    (t95542, t95545, t95550, t95556, t95566)
}
