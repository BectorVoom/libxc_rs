//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1127/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1127(t10935: f64, t2813: f64, t3446: f64, t3261: f64, t498: f64, t97: f64, t10648: f64, t10971: f64, t11564: f64, t10966: f64, t1103: f64, t269: f64, t955: f64) -> (f64, f64, f64, f64) {
    let t40603 = t3446 * t10935 * t2813;
    let t40630 = t97 * t3261 * t498;
    let t40642 = t10648 * t10971 * t11564;
    let t40659 = t10966 * t1103 * t955 * t269;
    (t40603, t40630, t40642, t40659)
}
