//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2435/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2435(t10970: f64, t820: f64, t1041: f64, t10868: f64, t248: f64, t2780: f64, t10277: f64, t976: f64, t11046: f64, t42387: f64, t10457: f64, t10936: f64, t3180: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42397 = t820 * t10970;
    let t42432 = t1041 * t248 * t10868 * t2780;
    let t42444 = t976 * t10277;
    let t42483 = t11046 * t42387;
    let t42488 = t820 * t10457;
    let t42496 = t3180 * t10936;
    (t42397, t42432, t42444, t42483, t42488, t42496)
}
