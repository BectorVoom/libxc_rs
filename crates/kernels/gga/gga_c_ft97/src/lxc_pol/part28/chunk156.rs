//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 156/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk156<F: Float>(t177: F, t178: F, t377: F, t381: F, t529: F, t637: F, t629: F, t631: F, t634: F, t184: F, t21: F, t19: F, t362: F) -> (F, F, F, F, F, F, F, F) {
    let t639 = F::cast_from(1.0_f64) / t178 / t177;
    let t641 = F::cast_from(0.14443083333333333333e0_f64) * t377;
    let t643 = F::cast_from(0.234754e0_f64) * t529 - t641 - F::cast_from(0.14443083333333333333e0_f64) * t381;
    let t645 = t637 * t639 * t643;
    let t648 = t629 + t631 * t634 / F::cast_from(6.0_f64) + t631 * t645 / F::cast_from(2.0_f64);
    let t649 = t648 * t184;
    let t650 = t649 * t21;
    let t920 = -t19 - t362;
    (t639, t641, t643, t645, t648, t649, t650, t920)
}
