//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 475/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk475(t1341: f64, t3732: f64, t1415: f64, t1411: f64, t10: f64, t79: f64) -> (f64, f64, f64, f64) {
    let t3733 = t1341 * t3732;
    let t3734 = t1415 * t3733;
    let t3735 = t1411 * t3734;
    let t3737 = t10 * t79;
    (t3733, t3734, t3735, t3737)
}
