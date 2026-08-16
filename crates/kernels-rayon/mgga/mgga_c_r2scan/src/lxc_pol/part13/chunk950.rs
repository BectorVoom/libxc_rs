//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 950/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk950(t10872: f64, t3300: f64, t2253: f64, t261: f64, t3299: f64, t2206: f64, t774: f64, t146: f64) -> (f64, f64, f64, f64, f64) {
    let t10873 = t10872 * t3300;
    let t10875 = t261 * t2253;
    let t10876 = t3299 * t10875;
    let t10878 = t2206 * t774;
    let t10879 = t146 * t10878;
    (t10873, t10875, t10876, t10878, t10879)
}
