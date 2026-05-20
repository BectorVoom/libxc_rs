//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1103/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1103<F: Float>(t13846: F, t1941: F, t241: F, t25981: F, t820: F, t197: F, t530: F, t2013: F, t8995: F, t2106: F, t9593: F, t198: F, t205: F, t2070: F) -> (F, F, F, F, F, F) {
    let t27932 = t1941 * t13846;
    let t27940 = t820 * t25981 * t241;
    let t28166 = t197 * t530;
    let t28167 = t2013 * t28166;
    let t28196 = t2013 * t8995;
    let t28286 = t2106 * t9593;
    let t28291 = t198 * t205 * t2070;
    (t27932, t27940, t28167, t28196, t28286, t28291)
}
