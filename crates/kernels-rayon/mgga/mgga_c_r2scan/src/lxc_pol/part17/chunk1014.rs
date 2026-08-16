//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1014/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1014(t3016: f64, t797: f64, t3060: f64, t11036: f64, t2928: f64, t2938: f64, t3358: f64, t1070: f64, t9640: f64, t3629: f64, t8358: f64, t6661: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12570 = t797 * t3016;
    let t12574 = t797 * t3060;
    let t12587 = t11036 * t2928;
    let t12589 = t3358 * t2938;
    let t12591 = t9640 * t1070;
    let t12593 = t8358 * t3629;
    let t12595 = t1070 * t2928;
    let t12596 = t6661 * t12595;
    (t12570, t12574, t12587, t12589, t12591, t12593, t12595, t12596)
}
