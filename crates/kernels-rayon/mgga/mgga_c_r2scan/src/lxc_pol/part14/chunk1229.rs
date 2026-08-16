//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1229/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1229(t40131: f64, t40137: f64, t38088: f64, t38093: f64, t40114: f64, t40117: f64, t40120: f64, t40123: f64, t40128: f64, t40134: f64, t40139: f64, t40142: f64) -> f64 {
    let t41709 = 0.18629842481251516498e0_f64 * t40131;
    let t41711 = 0.84755945902752848174e0_f64 * t40137;
    let t41714 = -0.87327386630866483588e-2_f64 * t40114 - 0.13099107994629972538e-1_f64 * t40117 - 0.13099107994629972538e-1_f64 * t40120 - 0.52396431978519890152e-1_f64 * t40123 - 0.46574606203128791246e-1_f64 * t38088 - 0.46574606203128791246e-1_f64 * t38093 - 0.43663693315433241794e-2_f64 * t40128 + t41709 + 0.87327386630866483588e-2_f64 * t40134 - t41711 - 0.26198215989259945076e-1_f64 * t40139 - 0.26198215989259945076e-1_f64 * t40142;
    t41714
}
