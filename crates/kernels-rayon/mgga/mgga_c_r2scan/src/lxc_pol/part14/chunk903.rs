//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 903/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk903(t1217: f64, t806: f64, t1218: f64, t2358: f64, t1216: f64, t298: f64, t2362: f64, t40: f64, t1000: f64, t6635: f64, t1257: f64, t1256: f64, t305: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8320 = t1217 * t806;
    let t8323 = t2358 * t1218;
    let t8326 = t298 * t1216;
    let t8329 = t2362 * t40;
    let t8336 = t6635 * t1000;
    let t8337 = t8336 * t1257;
    let t8340 = t305 * t1256;
    (t8320, t8323, t8326, t8329, t8337, t8340)
}
