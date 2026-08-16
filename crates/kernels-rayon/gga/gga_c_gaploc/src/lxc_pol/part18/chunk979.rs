//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 979/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk979(t10640: f64, t2508: f64, t2541: f64, t8682: f64, t3437: f64, t731: f64, t10631: f64, t10634: f64, t10638: f64, t9618: f64, t9620: f64, t9622: f64, t9627: f64, t9629: f64, t9632: f64, t9635: f64, t9651: f64) -> (f64, f64) {
    let t10642 = 0.46143157380853345701e-1_f64 * t2508 * t10640;
    let t10643 = t2541 * t8682;
    let t10645 = 0.53833683610995569986e-1_f64 * t2508 * t10643;
    let t10646 = t731 * t3437;
    let t10647 = 0.42725145723012357132e-3_f64 * t10646;
    let t10648 = t9618 - t9620 - t9622 - t9627 + t9629 + t9632 - t10631 + t10634 - t10638 - t10642 - t10645 - t10647 - t9635 - t9651;
    (t10643, t10648)
}
