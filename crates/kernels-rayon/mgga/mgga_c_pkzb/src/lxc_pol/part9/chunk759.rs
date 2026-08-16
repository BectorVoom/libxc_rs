//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 759/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk759(t158: f64, t5317: f64, t5318: f64, t5320: f64, t5345: f64, t133: f64, t1773: f64, t5181: f64, t568: f64, t614: f64, t1692: f64, t5217: f64, t596: f64) -> (f64, f64, f64, f64) {
    let t5348 = (t5317 + t5318 + t5320 + t5345) * t158;
    let t5356 = t133 * t1773;
    let t5357 = t5356 * t5181;
    let t5360 = t614 * t568;
    let t5361 = t5360 * t1692;
    let t5364 = t596 * t5217;
    (t5348, t5357, t5361, t5364)
}
