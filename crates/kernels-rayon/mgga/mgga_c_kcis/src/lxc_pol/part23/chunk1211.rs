//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1211/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1211(t16623: f64, t4288: f64, t27529: f64, t28640: f64, t17334: f64, t28624: f64, t17446: f64, t27544: f64, t5916: f64, t94748: f64, t12265: f64, t27543: f64, t6012: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97754 = t16623 * t4288;
    let t97756 = t28640 * t27529;
    let t97758 = t28624 * t17334;
    let t97760 = t27544 * t17446;
    let t97762 = t94748 * t5916;
    let t97765 = t12265 * t27543 * t6012;
    (t97754, t97756, t97758, t97760, t97762, t97765)
}
