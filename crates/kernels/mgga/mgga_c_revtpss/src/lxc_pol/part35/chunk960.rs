//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 960/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk960<F: Float>(t2018: F, t5617: F, t807: F, t241: F, t25981: F, t820: F, t25997: F, t5665: F, t1873: F, t26004: F, t1513: F, t25823: F, t1497: F, t1927: F, t1470: F, t2247: F) -> (F, F, F, F, F, F, F, F) {
    let t27936 = t2018 * t5617;
    let t27937 = t807 * t27936;
    let t27940 = t820 * t25981 * t241;
    let t27953 = t25997 * t5665;
    let t27955 = t26004 * t1873;
    let t28034 = t25823 * t1513;
    let t28150 = t1927 * t1497;
    let t28154 = t2247 * t1470;
    (t27936, t27937, t27940, t27953, t27955, t28034, t28150, t28154)
}
