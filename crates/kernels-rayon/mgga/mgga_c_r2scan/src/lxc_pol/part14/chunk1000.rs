//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1000/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1000(t6661: f64, t1010: f64, t263: f64, t826: f64, t3366: f64, t1276: f64, t1070: f64, t2391: f64, t3675: f64, t856: f64, t11189: f64, t11621: f64, param_eta: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11880 = t6661 * param_eta;
    let t11881 = t263 * t1010;
    let t11882 = t11881 * t826;
    let t11883 = t11880 * t11882;
    let t11885 = t3366 * t1010;
    let t11886 = t1276 * t11885;
    let t11888 = t1070 * t2391;
    let t11889 = t1276 * t11888;
    let t11993 = t3675 * t856;
    let t12024 = t11189 * t11621;
    (t11880, t11881, t11882, t11883, t11885, t11886, t11888, t11889, t11993, t12024)
}
