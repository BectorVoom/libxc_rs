//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2473/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2473(t48762: f64, t48765: f64, t48768: f64, t48770: f64, t49496: f64, t49499: f64, t49502: f64, t49506: f64, t49508: f64, t49510: f64, t49512: f64, t1068: f64, t11087: f64, t14662: f64, t3216: f64, t4700: f64, t4701: f64, t49068: f64, t49071: f64, t49075: f64, t49080: f64, t49517: f64, t49520: f64, t49522: f64, t49525: f64, t49529: f64) -> (f64, f64) {
    let t50757 = t48762 - t48765 - t48768 - t48770 + t49496 - t49499 + t49502 + t49506 - t49508 - t49510 + t49512;
    let t50764 = -3.0_f64 * t1068 * t14662 * t3216 * t4700 - t11087 * t4700 * t4701 + t49068 + t49071 + t49075 + t49080 - t49517 + t49520 + t49522 - t49525 - t49529;
    (t50757, t50764)
}
