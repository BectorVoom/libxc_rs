//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 757/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk757<F: Float>(t13421: F, t677: F, t25: F, t3817: F, t3762: F, t1113: F, t122: F, t1095: F, t2380: F, t200: F, t807: F, t2427: F, t6: F, t224: F, t2428: F, t3780: F) -> (F, F, F, F, F, F, F, F) {
    let t13422 = t677 * t13421;
    let t13425 = t3817 * t25;
    let t13426 = t13425 * t3762;
    let t13429 = t1113 * t122;
    let t13433 = t1095 * t2380;
    let t13434 = t13433 * t200;
    let t13435 = t807 * t13434;
    let t13442 = t2427 * t6;
    let t13443 = t224 * t13442;
    let t13444 = t3780 * t2428;
    (t13422, t13426, t13429, t13433, t13434, t13435, t13443, t13444)
}
