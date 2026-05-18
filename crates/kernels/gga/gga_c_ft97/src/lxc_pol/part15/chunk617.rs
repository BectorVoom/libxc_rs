//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 617/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk617<F: Float>(t1127: F, t2427: F, t677: F, t6: F, t224: F, t1113: F, t695: F, t122: F, t1095: F, t2378: F, t25: F, t2393: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13421 = t2427 * t1127;
    let t13422 = t677 * t13421;
    let t13442 = t2427 * t6;
    let t13443 = t224 * t13442;
    let t13463 = t695 * t1113;
    let t13467 = t695 * t122;
    let t13468 = t677 * t13467;
    let t13469 = t2378 * t1095;
    let t13473 = t695 * t25;
    let t13474 = t677 * t13473;
    let t13475 = t2393 * t1095;
    (t13421, t13422, t13443, t13463, t13467, t13468, t13469, t13473, t13474, t13475)
}
