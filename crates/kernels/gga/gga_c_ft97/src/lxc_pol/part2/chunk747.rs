//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 747/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk747<F: Float>(t13301: F, t3917: F, t1775: F, t3918: F, t3911: F, t1934: F, t3690: F) -> (F, F, F, F) {
    let t13302 = t3917 * t13301;
    let t13306 = 4.0 / 9.0 * t1775 * t3918;
    let t13308 = 4.0 / 27.0 * t1775 * t3911;
    let t13309 = t3690 * t1934;
    (t13302, t13306, t13308, t13309)
}
