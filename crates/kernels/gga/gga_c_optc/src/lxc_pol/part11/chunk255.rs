//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 255/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk255<F: Float>(t310: F, t888: F, t307: F, t306: F, t309: F, t300: F) -> (F, F, F, F) {
    let t889 = t310 * t888;
    let t891 = 0.18110753103726578864e-2 * t307 * t889;
    let t892 = t306 * t309;
    let t893 = t300 * t892;
    (t889, t891, t892, t893)
}
