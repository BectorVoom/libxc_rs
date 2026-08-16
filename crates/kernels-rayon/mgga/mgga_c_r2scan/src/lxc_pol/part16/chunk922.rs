//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 922/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk922(t2317: f64, t58: f64, t423: f64, t597: f64, t874: f64, t10680: f64, t120: f64, t518: f64, t3294: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10681 = t2317 * t58;
    let t10682 = t10681 * t423;
    let t10683 = t597 * t874;
    let t10684 = t10682 * t10683;
    let t10685 = t10680 * t10684;
    let t10697 = t120 * t518;
    let t10698 = t10697 * t3294;
    (t10681, t10682, t10683, t10684, t10685, t10697, t10698)
}
