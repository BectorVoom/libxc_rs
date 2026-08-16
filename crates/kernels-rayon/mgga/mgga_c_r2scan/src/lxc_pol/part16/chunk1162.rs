//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1162/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1162(t3269: f64, t42891: f64, t11479: f64, t3579: f64, t495: f64, t797: f64, t11518: f64, t11629: f64, t3262: f64, t9560: f64, t3275: f64, t3276: f64) -> (f64, f64, f64, f64) {
    let t42893 = t3269 * t42891 / 2.0_f64;
    let t42897 = t3579 * t495 * t11479 * t797 / 2.0_f64;
    let t42900 = 15.0_f64 / 8.0_f64 * t3262 * t11629 * t11518;
    let t42901 = t797 * t9560;
    let t42904 = 5.0_f64 / 16.0_f64 * t3275 * t3276 * t42901;
    (t42893, t42897, t42900, t42904)
}
