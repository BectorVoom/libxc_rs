//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1523/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1523(t16040: f64, t5250: f64, t3851: f64, t5335: f64, t12248: f64, t68: f64, t544: f64, t12250: f64, t3791: f64, t3793: f64, t1332: f64, t5333: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16041 = t16040 * t5250;
    let t16044 = t5335 * t3851;
    let t16046 = t68 * t12248;
    let t16047 = t544 * t16046;
    let t16048 = t12250 * t3791;
    let t16049 = t5335 * t16048;
    let t16052 = t5335 * t3793;
    let t16055 = t1332 * t5333;
    (t16041, t16044, t16047, t16048, t16049, t16052, t16055)
}
