//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 992/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk992(t30350: f64, t30379: f64, t1254: f64, t13682: f64, t30318: f64, t1275: f64, t7993: f64, t6100: f64, t2141: f64, t7976: f64, t4126: f64, t13561: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30380 = t30350 + t30379;
    let t30381 = t30380 * t1254;
    let t30384 = t30318 * t13682;
    let t30387 = t1275 * t7993;
    let t30388 = t6100 * t30387;
    let t30391 = t7976 * t2141;
    let t30393 = t4126 * t30391 * t1275;
    let t30396 = t13561 * t30391;
    (t30381, t30384, t30388, t30391, t30393, t30396)
}
