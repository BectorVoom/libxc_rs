//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1122/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1122<F: Float>(t21193: F, t3799: F, t41477: F, t420: F, t701: F, t88252: F, t2446: F, t88239: F, t18043: F, t5042: F, t21201: F, t1124: F, t79802: F) -> (F, F, F, F, F, F) {
    let t88562 = t3799 * t21193;
    let t88566 = t701 * t420 * t41477 * t88252;
    let t88570 = t701 * t420 * t2446 * t88239;
    let t88572 = t18043 * t5042;
    let t88575 = t3799 * t21201;
    let t88577 = t79802 * t1124;
    (t88562, t88566, t88570, t88572, t88575, t88577)
}
