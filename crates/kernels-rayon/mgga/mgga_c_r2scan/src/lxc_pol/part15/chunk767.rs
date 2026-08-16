//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 767/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk767(t1554: f64, t1632: f64, t551: f64, t574: f64, t2155: f64, t5174: f64, t2145: f64, t774: f64, t146: f64) -> (f64, f64, f64, f64) {
    let t6385 = t551 * t1632 * t1554;
    let t6386 = t574 * t6385;
    let t6392 = t2155 * t5174;
    let t6394 = t2145 * t774;
    let t6395 = t146 * t6394;
    (t6386, t6392, t6394, t6395)
}
