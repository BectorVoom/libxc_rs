//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1244/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1244(t1369: f64, t17946: f64, t136: f64, t238: f64, t1693: f64, t215: f64, t3683: f64, t3622: f64, t5547: f64, t17954: f64, t236: f64, t339: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19693 = t17946 * t1369;
    let t19695 = t238 * t136;
    let t19696 = t1693 * t19695;
    let t19697 = t215 * t3683;
    let t19698 = t19696 * t19697;
    let t19700 = t5547 * t3622;
    let t19703 = t339 * t17954 * t236;
    (t19693, t19695, t19696, t19697, t19698, t19700, t19703)
}
