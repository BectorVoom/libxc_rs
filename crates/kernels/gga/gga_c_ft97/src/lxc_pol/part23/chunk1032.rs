//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1032/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1032<F: Float>(t231: F, t5260: F, t6045: F, t5284: F, t27506: F, t6999: F, t1472: F, t25112: F, t25132: F, t28572: F, t28575: F, t28577: F, t28613: F, t28620: F, t28646: F, t30709: F, t30728: F, t31381: F, t6242: F, t6249: F, t7006: F, t7009: F, t7012: F) -> (F, F, F, F, F) {
    let t31385 = t231 * t5260;
    let t31386 = t6045 * t31385;
    let t31389 = t231 * t5284;
    let t31398 = t27506 * t6999;
    let t31408 = -0.80559205902449556552e-1 * t28572 - 0.66678001092592592595e-1 * t28575 + 0.80559205902449556552e-1 * t28577 + t25132 + 0.10001700163888888889e0 * t6249 * t6045 * t31381 - 0.10001700163888888889e0 * t6242 * t31386 - 0.30005100491666666667e0 * t25112 * t6045 * t31389 - 0.53342400874074074075e0 * t6249 * t27506 * t7012 + 0.14097861032928672397e1 * t7009 * t30728 + 0.53342400874074074075e0 * t6242 * t31398 - 0.14097861032928672397e1 * t7006 * t30728 + 0.66678001092592592595e-1 * t28613 - 0.11113000182098765433e-1 * t28620 + 0.88904001456790123461e-1 * t28646 - 0.48897200801234567903e0 * t1472 * t30709;
    (t31385, t31386, t31389, t31398, t31408)
}
