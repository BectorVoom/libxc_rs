//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 564/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk564(t511: f64, t7482: f64, t1971: f64, t1970: f64, t2106: f64, t261: f64) -> (f64, f64, f64) {
    let t7483 = t511 * t7482;
    let t7484 = t1971 * t7483;
    let t7485 = t1970 * t7484;
    let t7486 = 0.25538759935978703638e-4_f64 * t7485;
    let t7487 = t261 * t2106;
    (t7484, t7486, t7487)
}
