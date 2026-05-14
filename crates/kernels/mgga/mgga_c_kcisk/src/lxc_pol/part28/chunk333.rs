//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 333/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk333<F: Float>(t1894: F, t719: F, t735: F, t1935: F, t1757: F, t642: F, t734: F, t1862: F, t716: F, t740: F) -> (F, F, F, F, F, F, F, F) {
    let t1936 = t719 * t1894;
    let t1937 = t735 * t1936;
    let t1938 = t1935 * t1937;
    let t1940 = t642 * t1757;
    let t1941 = t735 * t1940;
    let t1942 = t734 * t1941;
    let t1944 = t1862 * t716;
    let t1945 = t1944 * t740;
    (t1936, t1937, t1938, t1940, t1941, t1942, t1944, t1945)
}
