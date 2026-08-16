//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 984/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk984(t11582: f64, t597: f64, t10649: f64, t10648: f64, t10681: f64, t897: f64, t10683: f64, t10680: f64, t10674: f64, t10676: f64, t10673: f64, t2810: f64, t3446: f64, t3453: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11583 = t11582 * t597;
    let t11584 = t10649 * t11583;
    let t11585 = t10648 * t11584;
    let t11587 = t10681 * t897;
    let t11588 = t11587 * t10683;
    let t11589 = t10680 * t11588;
    let t11591 = t10674 * t897;
    let t11592 = t11591 * t10676;
    let t11593 = t10673 * t11592;
    let t11598 = t3446 * t3453 * t2810;
    (t11583, t11584, t11585, t11587, t11588, t11589, t11591, t11592, t11593, t11598)
}
