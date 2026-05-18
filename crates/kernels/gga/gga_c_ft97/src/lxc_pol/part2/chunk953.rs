//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 953/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk953<F: Float>(t811: F, t820: F, t2724: F, t4125: F, t816: F, t13596: F, t13593: F, t13600: F, t13603: F, t13607: F, t13611: F, t13614: F, t13618: F, t9639: F, t9642: F, t9648: F) -> (F, F, F, F) {
    let t14770 = t811 * t820;
    let t14774 = t2724 * t4125;
    let t14781 = t816 * t4125;
    let t14788 = F::new(0.22226000364197530866e-1) * t13596;
    let t14798 = F::new(0.10001700163888888889e0) * t13593 - t14788 + F::new(0.14817333576131687243e-1) * t13600 + F::new(0.22226000364197530865e-1) * t13603 + F::new(0.51860667516460905352e-1) * t13607 - F::new(0.88904001456790123461e-1) * t13611 - F::new(0.33339000546296296298e-1) * t13614 + F::new(0.13335600218518518519e0) * t13618 - F::new(0.74086667880658436219e-2) * t9639 + F::new(0.55565000910493827163e-2) * t9648 + F::new(0.74086667880658436217e-2) * t9642;
    (t14770, t14774, t14781, t14798)
}
