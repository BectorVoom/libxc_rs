//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 903/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk903(t17899: f64, t2394: f64, t1127: f64, t3751: f64, t680: f64, t1096: f64, t3817: f64, t122: f64, t237: f64, t3758: f64, t1113: f64, t689: f64) -> (f64, f64, f64, f64, f64) {
    let t17933 = t2394 * t17899;
    let t17936 = t3751 * t1127;
    let t17937 = t680 * t17936;
    let t17941 = t680 * t1096 * t3817;
    let t17944 = t237 * t122;
    let t17945 = t3758 * t17944;
    let t17946 = t689 * t1113;
    (t17933, t17937, t17941, t17945, t17946)
}
