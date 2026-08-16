//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1050/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1050(t1017: f64, t525: f64, t1181: f64, t2068: f64, t7351: f64, t4773: f64, t570: f64, t10146: f64, t167: f64, t576: f64, t137: f64, t3300: f64) -> (f64, f64, f64, f64, f64) {
    let t34681 = t525 * t1017;
    let t34684 = t2068 * t1181 * t7351 * t34681;
    let t34686 = t570 * t4773;
    let t34691 = t576 * t167 * t10146;
    let t34692 = t3300 * t137;
    (t34681, t34684, t34686, t34691, t34692)
}
