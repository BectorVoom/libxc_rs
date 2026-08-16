//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 938/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk938(t10868: f64, t2150: f64, t2147: f64, t545: f64, t775: f64, t2206: f64, t774: f64, t146: f64, t2190: f64, t261: f64, t3299: f64, t2218: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10869 = t10868 * t2150;
    let t10870 = t2147 * t10869;
    let t10872 = t545 * t775;
    let t10878 = t2206 * t774;
    let t10879 = t146 * t10878;
    let t10885 = t261 * t2190;
    let t10886 = t3299 * t10885;
    let t10891 = t261 * t2218;
    (t10869, t10870, t10872, t10878, t10879, t10885, t10886, t10891)
}
