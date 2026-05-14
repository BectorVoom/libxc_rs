//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1313/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1313<F: Float>(t1856: F, t25940: F, t5802: F, t1899: F, t2786: F, t7443: F, t1893: F, t9235: F, t17577: F, t17579: F, t3524: F, t3551: F, t25918: F, t25921: F, t25924: F, t25927: F, t25930: F, t25933: F, t25936: F, t25939: F) -> (F, F, F, F, F, F) {
    let t25943 = 0.51726012919273400301e3 * t5802 * t25940 * t1856;
    let t25946 = 0.32163958997385070134e2 * t1899 * t2786 * t7443;
    let t25949 = 0.51726012919273400301e3 * t5802 * t9235 * t1893;
    let t25953 = 0.24955700379505800916e5 * t17577 * t3524 * t17579 * t1856;
    let t25956 = 6.0 * t1899 * t3551 * t1856;
    let t25957 = t25918 + t25921 - t25924 - t25927 - t25930 - t25933 - t25936 + t25939 + t25943 + t25946 + t25949 + t25953 + t25956;
    (t25943, t25946, t25949, t25953, t25956, t25957)
}
