//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1759/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1759(t25138: f64, t72: f64, t1927: f64, t6973: f64, t6977: f64, t2311: f64, t76: f64, t1926: f64, t10298: f64, t38: f64, t10309: f64, t6957: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25139 = t25138 * t72;
    let t25140 = t25139 * t1927;
    let t25143 = t6973 * t6977;
    let t25146 = t76 * t2311;
    let t25147 = t1926 * t25146;
    let t25150 = t10298 * t38;
    let t25157 = t10309 * t6957;
    (t25139, t25140, t25143, t25146, t25147, t25150, t25157)
}
