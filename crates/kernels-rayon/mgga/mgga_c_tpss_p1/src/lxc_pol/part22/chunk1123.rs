//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1123/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1123(t12410: f64, t3068: f64, t1562: f64, t2841: f64, t9702: f64, t1111: f64, t1125: f64, t12330: f64, t12355: f64, t12361: f64, t12363: f64, t12368: f64, t12371: f64, t12374: f64, t12381: f64, t12385: f64, t12391: f64, t12395: f64, t12401: f64, t12406: f64, t12409: f64, t3067: f64, t9556: f64, t9563: f64, t9573: f64, t9633: f64, t9658: f64, t9661: f64) -> f64 {
    let t12411 = t3068 * t12410;
    let t12414 = t1562 * t2841;
    let t12415 = t9702 * t12414;
    let t12421 = -t9556 * t12330 / 2304.0_f64 + t1111 * t12355 / 3072.0_f64 - t12361 - t1125 * t12363 / 4608.0_f64 + t12368 / 20736.0_f64 - t12371 - t3067 * t12374 / 1152.0_f64 + t9573 * t12381 / 2304.0_f64 + t12385 / 1296.0_f64 - t9556 * t12391 / 1152.0_f64 + 5.0_f64 / 6912.0_f64 * t3067 * t12395 - t9563 / 3456.0_f64 - t1125 * t12401 / 768.0_f64 + t12406 + t12409 + t9573 * t12411 / 4608.0_f64 + 5.0_f64 / 13824.0_f64 * t3067 * t12415 + t9633 / 648.0_f64 + t9658 / 648.0_f64 - t9661 / 864.0_f64;
    t12421
}
