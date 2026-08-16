//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1103/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1103(t18331: f64, t2970: f64, t2177: f64, t91: f64, t204: f64, t3981: f64, t824: f64) -> (f64, f64, f64) {
    let t18332 = t2970 * t18331;
    let t18406 = t2177 * t2177;
    let t18408 = 1.0_f64 / t91 / t18406;
    let t18427 = t204 * t3981 * t824;
    (t18332, t18408, t18427)
}
