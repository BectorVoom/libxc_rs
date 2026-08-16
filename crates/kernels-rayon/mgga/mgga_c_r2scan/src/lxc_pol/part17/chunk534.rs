//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 534/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk534(t1421: f64, t1459: f64, t1511: f64, t1513: f64, t1526: f64, t2810: f64, t2813: f64, t2816: f64, t2866: f64, t2869: f64, t2872: f64, t2879: f64, t881: f64) -> f64 {
    let t2881 = t2866 + t1421 - t1511 + t1459 + t2869 - t1526 - 0.2363e1_f64 * t881 * t2816 - 0.2363e1_f64 * t2872 - 0.2363e1_f64 * t881 * t2810 - 0.2363e1_f64 * t881 * t2813 - t1513 + t2879;
    t2881
}
