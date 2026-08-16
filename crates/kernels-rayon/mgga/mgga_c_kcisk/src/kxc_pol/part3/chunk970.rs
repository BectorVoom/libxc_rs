//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 970/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk970(t14315: f64, t4209: f64, t1512: f64, t4192: f64, t493: f64, t1481: f64, t3783: f64, t4211: f64, t1501: f64, t4185: f64, t1483: f64, t4241: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t14316 = t4209 * t14315;
    let t14317 = t1512 * t4192;
    let t14318 = t493 * t14317;
    let t14320 = t1481 * t3783;
    let t14321 = t14320 * sigma0;
    let t14322 = t14321 * t4211;
    let t14324 = t1501 * t4185;
    let t14326 = t1483 * t4241;
    (t14316, t14318, t14322, t14324, t14326)
}
