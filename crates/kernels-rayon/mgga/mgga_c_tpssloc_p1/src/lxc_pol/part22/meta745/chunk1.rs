//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2474/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2474(t1023: f64, t10413: f64, t14077: f64, t21516: f64, t21532: f64, t3039: f64, t3048: f64, t3070: f64, t3071: f64, t42483: f64, t42546: f64, t4347: f64, t4582: f64, t48611: f64, t48670: f64, t48674: f64, t5681: f64, t5867: f64, t5869: f64, t61866: f64, t70086: f64, t70122: f64, t70389: f64, t70391: f64) -> f64 {
    let t70396 = t42483 * t48611 * t70122 * t1023 / 1024.0_f64 + t48670 / 3456.0_f64 + t61866 / 768.0_f64 + t48674 / 5184.0_f64 - t42546 * t21532 / 1536.0_f64 + t3070 * t3071 * t5867 * t4347 / 1536.0_f64 + t10413 * t3071 * t5681 * t70086 / 768.0_f64 - t14077 * t5869 / 192.0_f64 - 5.0_f64 / 972.0_f64 * t3048 * t21516 + 5.0_f64 / 7776.0_f64 * t70389 - t3039 * t4582 * t70391 * t1023 / 3072.0_f64;
    t70396
}
