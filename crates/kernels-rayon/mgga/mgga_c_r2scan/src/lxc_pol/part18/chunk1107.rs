//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1107/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1107(t40041: f64, t38145: f64, t6093: f64, t7605: f64, t6085: f64, t8081: f64, t7619: f64, t2147: f64, t7624: f64, t1575: f64, t269: f64, t546: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40042 = 0.46574606203128791246e-1_f64 * t40041;
    let t40044 = t6093 * t38145 * t7605;
    let t40047 = t6085 * t38145 * t8081;
    let t40048 = 0.46574606203128791246e-1_f64 * t40047;
    let t40050 = t6093 * t38145 * t7619;
    let t40051 = 0.13972381860938637374e0_f64 * t40050;
    let t40053 = t2147 * t38145 * t7624;
    let t40054 = 0.46574606203128791246e-1_f64 * t40053;
    let t40061 = t1575 * t269;
    let t40062 = t546 * t40061;
    (t40042, t40044, t40048, t40051, t40054, t40061, t40062)
}
