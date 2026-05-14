//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 814/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk814<F: Float>(t13848: F, t7416: F, t12255: F, t769: F, t313: F, t39403: F, t12223: F, t2464: F, t2465: F, t825: F, t39002: F, t787: F, t9824: F, t13891: F, t2033: F, t549: F) -> (F, F, F, F, F, F, F) {
    let t47494 = t7416 * t13848;
    let t47496 = t769 * t12255;
    let t47500 = t313 * t39403;
    let t47506 = t825 * t2464 * t2465 * t12223;
    let t47508 = t787 * t39002;
    let t47509 = t47508 * t9824;
    let t47517 = t2033 * t549 * t13891;
    (t47494, t47496, t47500, t47506, t47508, t47509, t47517)
}
