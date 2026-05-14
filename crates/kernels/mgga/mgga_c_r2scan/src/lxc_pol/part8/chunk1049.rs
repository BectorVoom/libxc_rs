//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1049/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1049<F: Float>(t9863: F, t298: F, t994: F, t302: F, t1000: F, t2916: F, t6635: F, t2368: F, t2920: F, t308: F, t1001: F, t10410: F, t10413: F, t2911: F, t2917: F, t2921: F, t295: F, t305: F, t309: F, t6648: F, t997: F, tau1: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10416 = 3.0 * t9863;
    let t10417 = t298 * t10416;
    let t10420 = t994 * t994;
    let t10422 = 1.0 / t302 / t10420;
    let t10423 = tau1 * t10422;
    let t10432 = t2916 * t1000;
    let t10433 = t6635 * t10432;
    let t10436 = t2368 * t2920;
    let t10439 = -t10416;
    let t10440 = t308 * t10439;
    let t10443 = -10.0 / 27.0 * t295 * t10410 + 10.0 / 3.0 * t295 * t10413 + 5.0 / 3.0 * t295 * t10417 - 440.0 / 27.0 * t10423 * t309 + 200.0 / 9.0 * t2911 * t1001 - 50.0 / 9.0 * t997 * t2917 - 25.0 / 3.0 * t997 * t2921 - 10.0 / 27.0 * t305 * t10433 + 10.0 / 3.0 * t305 * t10436 + 5.0 / 3.0 * t305 * t10440 + t6648;
    (t10416, t10417, t10423, t10432, t10433, t10436, t10439, t10440, t10443)
}
