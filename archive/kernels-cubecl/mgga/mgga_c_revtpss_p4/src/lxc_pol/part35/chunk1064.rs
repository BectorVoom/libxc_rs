//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1064/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1064<F: Float>(t25986: F, t5609: F, t2661: F, t13846: F, t1941: F, t2018: F, t5617: F, t807: F, t241: F, t25981: F, t820: F, t25997: F, t5665: F) -> (F, F, F, F, F, F, F) {
    let t27928 = t25986 * t5609;
    let t27929 = t2661 * t27928;
    let t27932 = t1941 * t13846;
    let t27936 = t2018 * t5617;
    let t27937 = t807 * t27936;
    let t27940 = t820 * t25981 * t241;
    let t27953 = t25997 * t5665;
    (t27928, t27929, t27932, t27936, t27937, t27940, t27953)
}
