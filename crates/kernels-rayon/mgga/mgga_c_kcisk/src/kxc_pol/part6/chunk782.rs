//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 782/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk782(t14223: f64, t2237: f64, t2233: f64, t1337: f64, t2211: f64, t1336: f64, t140: f64, t2076: f64, t2869: f64) -> (f64, f64, f64, f64) {
    let t19020 = t14223 * t2237;
    let t19028 = t14223 * t2233;
    let t19053 = t1337 * t2211;
    let t19055 = t140 * t1336 * t19053;
    let t19100 = t2869 * t2076;
    (t19020, t19028, t19055, t19100)
}
