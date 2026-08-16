//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1071/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1071(t10734: f64, t254: f64, t255: f64, t6314: f64, t6321: f64, t1415: f64, t2116: f64, t5: f64, t511: f64, t57: f64, t7: f64, t2158: f64, t37699: f64) -> (f64, f64, f64) {
    let t37822 = t254 * t10734 * t6314 * t255 * t6321;
    let t37833 = t5 * t7 * t1415 * t511 * t57 * t2116;
    let t37835 = t37699 * t2158;
    (t37822, t37833, t37835)
}
