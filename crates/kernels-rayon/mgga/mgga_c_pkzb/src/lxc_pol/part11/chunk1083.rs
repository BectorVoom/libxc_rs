//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1083/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1083(t5931: f64, t785: f64, t5718: f64, t2177: f64, t91: f64, t204: f64, t3981: f64, t824: f64) -> (f64, f64, f64, f64) {
    let t18338 = t5931 * t785;
    let t18353 = t5718 * t785;
    let t18406 = t2177 * t2177;
    let t18408 = 1.0_f64 / t91 / t18406;
    let t18427 = t204 * t3981 * t824;
    (t18338, t18353, t18408, t18427)
}
