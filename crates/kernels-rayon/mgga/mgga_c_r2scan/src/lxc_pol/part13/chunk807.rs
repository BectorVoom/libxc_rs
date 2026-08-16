//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 807/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk807(t2271: f64, t2816: f64, t1376: f64, t955: f64, t2461: f64, t424: f64, t6801: f64, t6881: f64, t6885: f64, t6888: f64, t7117: f64, t7121: f64, t7126: f64, t7128: f64, t7129: f64, t881: f64) -> (f64, f64, f64) {
    let t7132 = 0.4726e1_f64 * t2271 * t2816;
    let t7133 = t1376 * t955;
    let t7136 = t424 * t2461;
    let t7139 = t7117 - t7121 + t6801 + t6881 - 0.4726e1_f64 * t6885 - 0.4726e1_f64 * t6888 - t7126 - t7128 - 0.2363e1_f64 * t7129 - t7132 - 0.2363e1_f64 * t881 * t7133 - 0.4726e1_f64 * t881 * t7136;
    (t7133, t7136, t7139)
}
