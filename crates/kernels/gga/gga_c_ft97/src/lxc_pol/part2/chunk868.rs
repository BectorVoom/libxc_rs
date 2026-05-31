//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 868/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk868<F: Float>(t807: F, t9542: F, t1092: F, t1771: F, t3740: F, t458: F, t3743: F, t11176: F, t3747: F, t13315: F, t9568: F, t92: F) -> (F, F, F, F, F, F, F, F) {
    let t13531 = t807 * t9542;
    let t13538 = t1771 * t1092;
    let t13540 = t458 * t3740;
    let t13541 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t13540;
    let t13542 = t458 * t3743;
    let t13543 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13542;
    let t13544 = t11176 * t3747;
    let t13546 = t9568 * t13315;
    let t13547 = t92 * t13546;
    (t13531, t13538, t13540, t13541, t13542, t13543, t13544, t13547)
}
