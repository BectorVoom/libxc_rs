//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1104/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1104(t31276: f64, t8875: f64, t1579: f64, t2095: f64, t355: f64, t171: f64, t5011: f64, t31443: f64, t35296: f64, t1017: f64, t2030: f64, t2297: f64, t8927: f64) -> (f64, f64, f64, f64) {
    let t35643 = t31276 * t8875;
    let t35646 = t2095 * t1579 * t355;
    let t35649 = t171 * t5011;
    let t35651 = t31443 * t35649 * t35296;
    let t35656 = t2030 * t8927 * t2297 * t1017;
    (t35643, t35646, t35651, t35656)
}
