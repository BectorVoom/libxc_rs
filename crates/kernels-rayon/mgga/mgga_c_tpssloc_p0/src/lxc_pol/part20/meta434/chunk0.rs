//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1855/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1855(t1003: f64, t1058: f64, t1061: f64, t1063: f64, t11037: f64, t11046: f64, t13940: f64, t14615: f64, t14618: f64, t14623: f64, t14627: f64, t14631: f64, t14640: f64, t14645: f64, t14648: f64, t14651: f64, t14654: f64, t1610: f64, t3180: f64, t3186: f64, t3189: f64, t3197: f64, t3200: f64, t3204: f64, t353: f64, t384: f64, t4615: f64, t4669: f64, t4685: f64, t4689: f64, t4691: f64) -> f64 {
    let t14657 = 2.0_f64 * t1003 * t4691 + 2.0_f64 * t1058 * t14645 + 2.0_f64 * t1061 * t14651 + 2.0_f64 * t1063 * t4615 - 2.0_f64 * t11037 * t4685 + t11046 * t14631 + t13940 * t384 - 2.0_f64 * t14615 * t3200 + 2.0_f64 * t14618 * t3189 - t14623 * t3200 - t14627 * t3200 + t14640 * t353 + 2.0_f64 * t14648 * t3186 + 2.0_f64 * t14654 * t3186 + t1610 * t3204 + 2.0_f64 * t3180 * t4689 + t3197 * t4669;
    t14657
}
