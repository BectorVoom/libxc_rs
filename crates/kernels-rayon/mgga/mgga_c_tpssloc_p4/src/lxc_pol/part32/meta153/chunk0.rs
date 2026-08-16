//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 811/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk811(t4166: f64, t816: f64, t1500: f64, t838: f64, t842: f64, t242: f64, t2628: f64, t812: f64, t244: f64, t67: f64, t246: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4167 = t4166 * t816;
    let t4170 = t1500 * t838;
    let t4172 = t4166 * t842;
    let t4177 = t2628 * t242;
    let t4178 = t812 * t4177;
    let t4179 = t244 * t67;
    let t4180 = t4179 * t246;
    (t4167, t4170, t4172, t4177, t4178, t4179, t4180)
}
