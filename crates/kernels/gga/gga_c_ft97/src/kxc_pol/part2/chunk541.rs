//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 541/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk541<F: Float>(t2481: F, t2482: F, t2484: F, t3139: F, t3908: F, t3911: F, t3914: F, t3918: F, t3922: F, t3925: F, t3927: F, t3932: F, t3936: F, t462: F, t92: F, t734: F, t91: F) -> (F, F) {
    let t3938 = t2481 + t2482 / 9.0 + t2484 / 3.0 + t3908 / 9.0 - 2.0 / 9.0 * t462 * t3911 + t462 * t3914 / 3.0 + 2.0 / 3.0 * t462 * t3918 + 2.0 / 3.0 * t3139 * t3922 + t3925 / 3.0 + t462 * t3927 / 3.0 + 2.0 * t462 * t3932 - t92 * t3936;
    let t3940 = t91 * t734 * t3938;
    (t3938, t3940)
}
