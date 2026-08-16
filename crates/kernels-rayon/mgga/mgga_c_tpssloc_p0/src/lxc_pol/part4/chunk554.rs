//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 554/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk554(t2887: f64, t2764: f64, t938: f64, t942: f64, t320: f64, t941: f64) -> (f64, f64, f64, f64) {
    let t2888 = 1.0_f64 / t2887;
    let t2892 = 0.12361111111111111111e-1_f64 * t2764;
    let t2900 = t938 * t942;
    let t2903 = t941 * t320;
    let t2904 = 1.0_f64 / t2903;
    (t2888, t2892, t2900, t2904)
}
