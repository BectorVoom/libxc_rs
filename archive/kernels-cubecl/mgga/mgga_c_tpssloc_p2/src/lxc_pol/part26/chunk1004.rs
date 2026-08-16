//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1004/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1004<F: Float>(t11778: F, t61: F, t11148: F, t248: F, t121: F, t3584: F, t3243: F, t1227: F, t1229: F, t676: F, t1090: F, t3536: F, t3572: F) -> (F, F, F, F, F, F) {
    let t11779 = t61 * t11778;
    let t11781 = t248 * t11779 * t11148;
    let t11784 = t121 * t3584;
    let t11786 = t248 * t11784 * t3243;
    let t11787 = t1227 * t11786;
    let t11789 = t676 * t1229;
    let t11791 = t248 * t11789 * t1090;
    let t11792 = t1227 * t11791;
    let t11794 = t3536 * t3572;
    (t11781, t11786, t11787, t11791, t11792, t11794)
}
