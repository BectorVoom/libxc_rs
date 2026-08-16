//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 234/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk234(t153: f64, t812: f64, t137: f64, t747: f64, t161: f64, t774: f64, t755: f64, t159: f64, t8: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t813 = t153 * t812;
    let t815 = t747 * t137;
    let t816 = t815 * t161;
    let t818 = t161 * t774;
    let t819 = t755 * t818;
    let t821 = t159 * t8;
    let t822 = 1.0_f64 / t821;
    (t813, t815, t816, t818, t819, t821, t822)
}
