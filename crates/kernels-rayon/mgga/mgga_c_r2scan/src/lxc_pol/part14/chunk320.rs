//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 320/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk320(t322: f64, t1115: f64, t499: f64, t1048: f64, t1072: f64) -> (f64, f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t1116 = t499 * t1115;
    let t1118 = t1048 * t1116 / 4.0_f64;
    let t1119 = t1072 / 4.0_f64;
    let t1120 = piecewise3(t324, 0.0_f64, t1119);
    (t1118, t1119, t1120)
}
