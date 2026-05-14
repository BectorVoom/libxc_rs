//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1386/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1386<F: Float>(t126455: F, t24976: F, t6317: F, t126463: F, t24981: F, t126467: F, t28772: F, t4917: F, t856: F, t24980: F, t25037: F, t126447: F, t2665: F, t446: F, t10409: F, t126451: F) -> (F, F, F, F, F, F, F) {
    let t127855 = t6317 * t24976 * t126455;
    let t127858 = t6317 * t24981 * t126463;
    let t127861 = t6317 * t28772 * t126467;
    let t127863 = t4917 * t856;
    let t127866 = t24980 * t28772 * t25037 * t127863;
    let t127869 = t446 * t2665 * t126447;
    let t127872 = t446 * t10409 * t126451;
    (t127855, t127858, t127861, t127863, t127866, t127869, t127872)
}
