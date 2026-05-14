//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 789/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk789<F: Float>(t379: F, t401: F, t422: F, t1595: F, t7857: F, t11120: F, t5576: F, t73: F, t1669: F, t5597: F) -> (F, F, F, F, F) {
    let t22543 = t422 * t401 * t379;
    let t22547 = t7857 * t1595;
    let t22548 = t22547 * t11120;
    let t22549 = t5576 * t73;
    let t22552 = t1669 * t5597;
    (t22543, t22547, t22548, t22549, t22552)
}
