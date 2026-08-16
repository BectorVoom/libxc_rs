//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1069/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1069(t37935: f64, t546: f64, t565: f64, t10734: f64, t547: f64, t10737: f64, t255: f64, t6319: f64) -> (f64, f64, f64, f64, f64) {
    let t37936 = t546 * t37935;
    let t37939 = t565 * t37935;
    let t37942 = t547 * t10734;
    let t37943 = t546 * t37942;
    let t37945 = t10737 * t255 * t6319;
    (t37936, t37939, t37942, t37943, t37945)
}
