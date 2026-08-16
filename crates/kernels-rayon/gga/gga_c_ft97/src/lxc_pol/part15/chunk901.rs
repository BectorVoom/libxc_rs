//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 901/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk901(t1221: f64, t3281: f64, t1225: f64, t1242: f64, t89: f64, t9555: f64, t1250: f64, t10491: f64, t1240: f64, t4423: f64, t8232: f64, t4426: f64, t7773: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56264 = t3281 * t1221;
    let t56665 = t3281 * t1225;
    let t56689 = t89 * t9555 * t1242;
    let t56957 = t3281 * t1250;
    let t57089 = t10491 * t1240;
    let t57435 = t8232 * t4423;
    let t57491 = t89 * t7773 * t4426;
    (t56264, t56665, t56689, t56957, t57089, t57435, t57491)
}
