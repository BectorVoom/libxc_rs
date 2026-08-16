//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 605/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk605(t5174: f64, t673: f64, t716: f64, t720: f64, t415: f64, t1797: f64, t704: f64, t1336: f64, t140: f64) -> (f64, f64, f64, f64) {
    let t5175 = t673 * t5174;
    let t5176 = t5175 * t716;
    let t5177 = t5176 * t720;
    let t5178 = t415 * t5177;
    let t5180 = t1797 * t704;
    let t5182 = t140 * t1336 * t5180;
    (t5177, t5178, t5180, t5182)
}
