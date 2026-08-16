//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1904/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1904(t26197: f64, t80670: f64, t1307: f64, t22635: f64, t26331: f64, t5187: f64, t567: f64, t26332: f64, t3719: f64, t1834: f64, t213: f64, t225: f64) -> (f64, f64, f64, f64) {
    let t90551 = t80670 * t26197;
    let t90556 = t26331 * t22635 * t567 * t5187 * t1307;
    let t90560 = t26331 * t22635 * t26332 * t3719;
    let t90566 = t213 * t1834 * t225;
    (t90551, t90556, t90560, t90566)
}
