//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 651/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk651(t531: f64, t549: f64, t240: f64, t72: f64, t1386: f64, t2482: f64, t27: f64) -> (f64, f64, f64, f64) {
    let t4010 = 1.0_f64 / t549 / t531;
    let t4011 = t240 * t4010;
    let t4012 = t4011 * t72;
    let t4018 = t2482 * t1386 * t27;
    (t4010, t4011, t4012, t4018)
}
