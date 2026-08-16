//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1306/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1306(t26763: f64, t7030: f64, t2389: f64, t8229: f64, t8331: f64, t34239: f64, t4391: f64, t6964: f64, t10525: f64, t10526: f64, t6689: f64, t8411: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34306 = t26763 * t7030;
    let t34307 = 0.29792074959875355558e-1_f64 * t34306;
    let t34308 = t8229 * t2389;
    let t34309 = 0.59584149919750711116e-1_f64 * t34308;
    let t34310 = t8331 * t2389;
    let t34311 = 0.59584149919750711116e-1_f64 * t34310;
    let t34314 = 0.85801175884441024006e1_f64 * t4391 * t6964 * t34239;
    let t34318 = 0.42900587942220512002e1_f64 * t10525 * t10526 * t34239;
    let t34320 = 0.10725146985555128001e1_f64 * t8411 * t6689;
    (t34307, t34309, t34311, t34314, t34318, t34320)
}
