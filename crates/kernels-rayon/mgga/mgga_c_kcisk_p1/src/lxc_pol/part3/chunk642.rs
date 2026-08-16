//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 642/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk642(t747: f64, t79: f64, t260: f64, t604: f64, t67: f64, t41: f64, t4971: f64, t1001: f64, t167: f64, t2689: f64, t1049: f64, t116: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7430 = t79 * t747;
    let t7567 = t260 * t67 * t604;
    let t7568 = t41 * t4971;
    let t9345 = t167 * t1001;
    let t9352 = t2689 * t1001;
    let t9355 = t116 * t1049;
    (t7430, t7567, t7568, t9345, t9352, t9355)
}
