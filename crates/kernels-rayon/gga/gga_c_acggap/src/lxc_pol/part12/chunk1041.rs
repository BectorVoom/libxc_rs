//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1041/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1041(t142: f64, t4479: f64, t8888: f64, t5129: f64, t7647: f64, t5133: f64, t2001: f64, t4518: f64, t4667: f64, t5267: f64, t5096: f64, t5101: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34532 = t8888 * t142 * t4479;
    let t34534 = t7647 * t5129;
    let t34537 = t7647 * t5133;
    let t34539 = t2001 * t4518;
    let t34541 = t2001 * t4667;
    let t34543 = t2001 * t5267;
    let t34545 = t2001 * t5096;
    let t34547 = t7741 * t5101;
    (t34532, t34534, t34537, t34539, t34541, t34543, t34545, t34547)
}
