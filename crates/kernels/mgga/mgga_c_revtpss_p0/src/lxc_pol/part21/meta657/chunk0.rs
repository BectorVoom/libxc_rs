//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2447/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2447<F: Float>(t3105: F, t3223: F, t11960: F, t351: F, t361: F, t369: F, t1041: F, t11262: F, t3135: F, t1033: F, t1036: F, t1038: F) -> (F, F, F, F) {
    let t42571 = t3223 * t3105;
    let t42576 = t351 * t361 * t11960 * t369;
    let t42580 = t1041 * t11262 * t3135;
    let t42584 = t1033 * t1036 * t11960 * t1038;
    (t42571, t42576, t42580, t42584)
}
