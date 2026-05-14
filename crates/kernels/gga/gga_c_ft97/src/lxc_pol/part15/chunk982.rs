//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 982/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk982<F: Float>(t21187: F, t3799: F, t21183: F, t41458: F, t420: F, t701: F, t88252: F, t2441: F, t88239: F, t41537: F, t704: F, t86571: F, t21193: F, t41477: F, t2446: F, t18043: F, t5042: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t88542 = t3799 * t21187;
    let t88544 = t3799 * t21183;
    let t88548 = t701 * t420 * t41458 * t88252;
    let t88552 = t701 * t420 * t2441 * t88239;
    let t88556 = t701 * t420 * t41537 * t88252;
    let t88560 = t701 * t420 * t704 * t86571;
    let t88562 = t3799 * t21193;
    let t88566 = t701 * t420 * t41477 * t88252;
    let t88570 = t701 * t420 * t2446 * t88239;
    let t88572 = t18043 * t5042;
    (t88542, t88544, t88548, t88552, t88556, t88560, t88562, t88566, t88570, t88572)
}
