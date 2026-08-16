//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1302/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1302(t113082: f64, t118413: f64, t118454: f64, t118466: f64, t118953: f64, t16596: f64, t1877: f64, t23290: f64, t23295: f64, t2522: f64, t25353: f64, t25365: f64, t25374: f64, t30770: f64, t4119: f64, t4255: f64, t4303: f64, t4314: f64, t6670: f64, t7540: f64, t8370: f64) -> f64 {
    let t119676 = -6.0_f64 * t113082 * t1877 * t25374 + 4.0_f64 * t118413 * t1877 * t23295 - 6.0_f64 * t118454 * t2522 * t6670 - 6.0_f64 * t118466 * t2522 * t6670 + 4.0_f64 * t118953 * t1877 * t23295 + 6.0_f64 * t16596 * t2522 * t30770 - 2.0_f64 * t1877 * t23290 * t7540 - 2.0_f64 * t1877 * t25353 * t6670 + 2.0_f64 * t1877 * t30770 * t4303 + 6.0_f64 * t2522 * t25365 * t30770 - 3.0_f64 * t2522 * t4119 * t8370 - 6.0_f64 * t4255 * t4314 * t8370;
    t119676
}
