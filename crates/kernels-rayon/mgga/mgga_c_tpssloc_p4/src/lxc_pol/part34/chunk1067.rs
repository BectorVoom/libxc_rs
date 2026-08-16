//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1067/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1067(t29025: f64, t29039: f64, t235: f64, t5617: f64, t7101: f64, t1499: f64, t2051: f64, t226: f64, t24265: f64, t25277: f64, t25293: f64, t25310: f64, t25317: f64, t28420: f64, t28424: f64, t28428: f64, t29010: f64, t5575: f64, t7839: f64, t812: f64) -> (f64, f64, f64, f64) {
    let t29040 = t29025 + t29039;
    let t29041 = t235 * t29040;
    let t29052 = t7101 * t5617;
    let t29054 = -t812 * t29010 - t24265 + 0.76763589786250567036e-1_f64 * t25277 + t226 * t29041 + 2.0_f64 * t1499 * t7839 - 0.76763589786250567036e-1_f64 * t25293 + t5575 * t2051 + 0.15352717957250113407e0_f64 * t25310 + 0.3289868133696452873e-1_f64 * t25317 - 0.3289868133696452873e-1_f64 * t28420 - 0.16449340668482264365e-1_f64 * t28424 + 0.3289868133696452873e-1_f64 * t28428 - t812 * t29052;
    (t29040, t29041, t29052, t29054)
}
