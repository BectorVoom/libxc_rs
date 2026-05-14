//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1218/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1218<F: Float>(t24265: F, t30607: F, t697: F, t27609: F, t30603: F, t1113: F, t668: F, t12: F, t14: F, t4995: F, t27669: F, t79528: F, t66493: F, t689: F, t13475: F, t3751: F) -> (F, F, F, F, F, F, F) {
    let t122987 = t24265 * t697 * t30607;
    let t122990 = t27609 * t697 * t30603;
    let t123006 = t1113 * t668;
    let t123015 = t12 * t4995 * t14;
    let t123028 = t79528 * t27669;
    let t123035 = t66493 * t689;
    let t123039 = t13475 * t3751;
    (t122987, t122990, t123006, t123015, t123028, t123035, t123039)
}
