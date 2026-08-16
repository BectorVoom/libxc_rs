//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1194/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1194(t11670: f64, t2124: f64, t29500: f64, t10868: f64, t2147: f64, t9445: f64, t10760: f64, t30320: f64, t30468: f64, t6085: f64, t29783: f64, t6093: f64) -> (f64, f64, f64, f64, f64) {
    let t43657 = t11670 * t2124 * t29500;
    let t43660 = t2147 * t10868 * t9445;
    let t43664 = t2147 * t10760 * t30320;
    let t43667 = t6085 * t10760 * t30468;
    let t43670 = t6093 * t10760 * t29783;
    (t43657, t43660, t43664, t43667, t43670)
}
