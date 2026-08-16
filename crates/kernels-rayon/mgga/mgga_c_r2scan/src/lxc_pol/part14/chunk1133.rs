//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1133/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1133(t2833: f64, t545: f64, t3300: f64, t10875: f64, t11744: f64, t146: f64, t2206: f64, t2832: f64, t3305: f64, t10760: f64, t22820: f64, t25697: f64) -> (f64, f64, f64, f64) {
    let t39739 = t545 * t2833;
    let t39740 = t39739 * t3300;
    let t39742 = t11744 * t10875;
    let t39745 = t146 * t2206 * t2832;
    let t39746 = t39745 * t3305;
    let t39749 = t22820 * t10760 * t25697;
    (t39740, t39742, t39746, t39749)
}
