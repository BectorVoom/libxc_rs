//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 991/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk991(t11880: f64, t11882: f64, t1010: f64, t3366: f64, t1276: f64, t1070: f64, t2391: f64, t11032: f64, t11034: f64, t11045: f64, t11051: f64, t11058: f64, t11866: f64, t11868: f64, t11870: f64, t11872: f64, t11874: f64, t11876: f64, t11878: f64) -> (f64, f64, f64, f64) {
    let t11883 = t11880 * t11882;
    let t11885 = t3366 * t1010;
    let t11886 = t1276 * t11885;
    let t11888 = t1070 * t2391;
    let t11889 = t1276 * t11888;
    let t11893 = -t11032 - t11034 / 3.0_f64 - t11866 / 3.0_f64 - t11868 / 4.0_f64 + t11870 / 8.0_f64 - t11872 / 8.0_f64 + t11874 / 4.0_f64 + t11876 / 3.0_f64 + t11878 / 4.0_f64 - 3.0_f64 / 4.0_f64 * t11883 - 2.0_f64 / 3.0_f64 * t11886 + t11889 / 4.0_f64 + t11045 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t11051 - t11058;
    (t11885, t11886, t11888, t11893)
}
