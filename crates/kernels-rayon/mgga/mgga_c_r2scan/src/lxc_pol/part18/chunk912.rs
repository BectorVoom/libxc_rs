//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 912/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk912(t2271: f64, t3165: f64, t6881: f64, t6888: f64, t7126: f64, t7129: f64, t7132: f64, t8649: f64, t8651: f64, t8652: f64, t881: f64, t9056: f64, t9787: f64, t9791: f64) -> f64 {
    let t9816 = t2271 * t3165;
    let t9818 = -0.2363e1_f64 * t881 * t9056 - t9787 - t8649 + t8651 + t6881 - t9791 - 0.2363e1_f64 * t6888 - t7126 - t8652 - 0.4726e1_f64 * t7129 - t7132 - 0.4726e1_f64 * t9816;
    t9818
}
