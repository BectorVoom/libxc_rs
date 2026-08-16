//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 656/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk656(t10450: f64, t1801: f64, t1800: f64, t1799: f64, t4581: f64, t5055: f64, t5054: f64, t1849: f64, t579: f64, t1336: f64, t140: f64, t4596: f64, t694: f64) -> (f64, f64, f64, f64, f64) {
    let t10451 = t1801 * t10450;
    let t10452 = t1800 * t10451;
    let t10453 = t1799 * t10452;
    let t10455 = t4581 * t5055;
    let t10456 = t5054 * t10455;
    let t10459 = 1.0_f64 / t579 / t1849;
    let t10461 = t140 * t1336 * t10459;
    let t10463 = 1.0_f64 / t4596 / t694;
    (t10453, t10456, t10459, t10461, t10463)
}
