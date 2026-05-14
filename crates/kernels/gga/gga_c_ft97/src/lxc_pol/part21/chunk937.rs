//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 937/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk937<F: Float>(t942: F, t965: F, t1871: F, t5675: F, t22952: F, t29569: F, t370: F, t27: F, t89: F, t4495: F, t5507: F, t28: F, t25528: F, t22883: F, t4436: F, t1307: F, t4505: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t29669 = t965 * t942;
    let t29671 = t1871 * t5675 * t29669;
    let t29672 = t22952 * t29671;
    let t29674 = t370 * t29569;
    let t29676 = t89 * t27 * t29674;
    let t29678 = t5507 * t4495;
    let t29679 = t28 * t29678;
    let t29680 = t89 * t29679;
    let t29682 = t25528 * t942;
    let t29683 = t28 * t29682;
    let t29684 = t89 * t29683;
    let t29688 = t22883 * t4436;
    let t29689 = t28 * t29688;
    let t29690 = t89 * t29689;
    let t29692 = t1307 * t4505;
    (t29669, t29671, t29672, t29674, t29676, t29678, t29680, t29682, t29684, t29688, t29690, t29692)
}
