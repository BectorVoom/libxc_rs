//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1085/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1085(t10922: f64, t10946: f64, t10992: f64, t158: f64, t2312: f64, t3446: f64, t37428: f64, t3428: f64, t3430: f64, t6836: f64, t10810: f64, t870: f64) -> (f64, f64, f64, f64) {
    let t38341 = t10922 * t10946;
    let t38346 = t3446 * t10992 * t158 * t37428 * t2312;
    let t38349 = t6836 * t3428 * t3430;
    let t38355 = t870 * t10810;
    (t38341, t38346, t38349, t38355)
}
