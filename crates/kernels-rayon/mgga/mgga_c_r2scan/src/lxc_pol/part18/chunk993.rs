//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 993/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk993(t322: f64, t1018: f64, t3381: f64, t1079: f64, t2405: f64, t11893: f64) -> (f64, f64, f64) {
    let t332 = 0.25e1_f64 < t322;
    let t11924 = t3381 * t1018;
    let t11926 = t1079 * t2405;
    let t11930 = piecewise3(t332, 0.0_f64, t11893);
    (t11924, t11926, t11930)
}
