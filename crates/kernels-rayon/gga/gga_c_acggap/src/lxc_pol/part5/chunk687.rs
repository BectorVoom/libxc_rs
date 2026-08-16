//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 687/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk687(t1055: f64, t4818: f64, t345: f64, t495: f64, t839: f64, t3579: f64, t4798: f64, t4800: f64, t4802: f64, t4804: f64, t4809: f64, t4812: f64, t4814: f64, t4817: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4819 = t1055 * t4818;
    let t4820 = t345 * t4819;
    let t4822 = t495 * t839;
    let t4823 = t1055 * t4822;
    let t4824 = t345 * t4823;
    let t4826 = t4798 + t4800 - 0.36675e0_f64 * t4802 + 0.2445e0_f64 * t4804 - t4809 - 0.12225e0_f64 * t4812 - 0.1141e1_f64 * t4814 - t4817 + 0.1467e1_f64 * t4820 + 0.7335e0_f64 * t4824 + t3579;
    (t4819, t4820, t4822, t4823, t4824, t4826)
}
