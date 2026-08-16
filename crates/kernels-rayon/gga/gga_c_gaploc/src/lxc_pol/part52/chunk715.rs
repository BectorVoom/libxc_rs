//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 715/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk715(t14266: f64, t569: f64, t568: f64, t600: f64, t13379: f64, t13381: f64, t13385: f64, t13389: f64, t13390: f64, t13395: f64, t13399: f64, t13405: f64, t13780: f64, t13783: f64, t574: f64, t597: f64) -> (f64, f64, f64, f64, f64) {
    let t14318 = t569 * t14266;
    let t14319 = t568 * t14318;
    let t14322 = t600 * t14266;
    let t14323 = t568 * t14322;
    let t14326 = -0.76685851907841499354e0_f64 * t13780 + 0.76685851907841499354e0_f64 * t13783 + t13379 + t13381 + t13385 - t13389 - 0.44688112439813033337e-1_f64 * t13390 + t13395 + 0.9585731488480187419e0_f64 * t13399 - t13405 - 0.23005755572352449806e1_f64 * t574 * t14319 + 0.23005755572352449806e1_f64 * t597 * t14323;
    (t14318, t14319, t14322, t14323, t14326)
}
