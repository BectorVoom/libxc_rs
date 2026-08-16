//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 953/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk953(t811: f64, t820: f64, t2724: f64, t4125: f64, t816: f64, t13596: f64, t13593: f64, t13600: f64, t13603: f64, t13607: f64, t13611: f64, t13614: f64, t13618: f64, t9639: f64, t9642: f64, t9648: f64) -> (f64, f64, f64, f64) {
    let t14770 = t811 * t820;
    let t14774 = t2724 * t4125;
    let t14781 = t816 * t4125;
    let t14788 = 0.22226000364197530866e-1_f64 * t13596;
    let t14798 = 0.10001700163888888889e0_f64 * t13593 - t14788 + 0.14817333576131687243e-1_f64 * t13600 + 0.22226000364197530865e-1_f64 * t13603 + 0.51860667516460905352e-1_f64 * t13607 - 0.88904001456790123461e-1_f64 * t13611 - 0.33339000546296296298e-1_f64 * t13614 + 0.13335600218518518519e0_f64 * t13618 - 0.74086667880658436219e-2_f64 * t9639 + 0.55565000910493827163e-2_f64 * t9648 + 0.74086667880658436217e-2_f64 * t9642;
    (t14770, t14774, t14781, t14798)
}
