//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1513/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1513(t225: f64, t5211: f64, t1332: f64, t5343: f64, t1372: f64, t1824: f64, t5286: f64, t562: f64, t12248: f64, t68: f64, t544: f64, t5333: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16030 = t5211 * t225;
    let t16033 = t1332 * t5343;
    let t16036 = t1372 * t1824;
    let t16040 = t562 * t5286;
    let t16046 = t68 * t12248;
    let t16047 = t544 * t16046;
    let t16055 = t1332 * t5333;
    (t16030, t16033, t16036, t16040, t16046, t16047, t16055)
}
