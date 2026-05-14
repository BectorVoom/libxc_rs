//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 799/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk799<F: Float>(t14175: F, t14176: F, t737: F, t762: F, t2608: F, t3699: F, t2486: F, t3690: F, t1091: F, t2579: F, t10007: F, t2492: F, t265: F, t13702: F, t9802: F, t13706: F) -> (F, F, F, F, F, F) {
    let t14177 = t14175 * t14176;
    let t14182 = t737 * t762;
    let t14183 = t3699 * t2608;
    let t14184 = t14182 * t14183;
    let t14187 = t2486 * t762;
    let t14188 = t3690 * t2608;
    let t14189 = t14187 * t14188;
    let t14192 = t1091 * t2579;
    let t14193 = t10007 * t14192;
    let t14196 = t2492 * t265;
    let t14197 = t14196 * t13702;
    let t14200 = t9802 * t265;
    let t14201 = t14200 * t13706;
    (t14177, t14184, t14189, t14193, t14197, t14201)
}
