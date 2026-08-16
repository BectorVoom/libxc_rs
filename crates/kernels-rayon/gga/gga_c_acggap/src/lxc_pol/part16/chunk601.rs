//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 601/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk601(t1055: f64, t5651: f64, t345: f64, t1713: f64, t355: f64, t721: f64, t3115: f64, t1734: f64, t1060: f64, t346: f64, t5506: f64, t4798: f64, t4800: f64, t4804: f64, t5639: f64, t5643: f64, t5647: f64, t5649: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5652 = t1055 * t5651;
    let t5653 = t345 * t5652;
    let t5655 = t355 * t1713;
    let t5656 = t5655 * t721;
    let t5657 = t3115 * t5656;
    let t5659 = t355 * t1734;
    let t5660 = t5659 * t721;
    let t5661 = t1060 * t5660;
    let t5663 = t346 * t5506;
    let t5664 = t345 * t5663;
    let t5667 = -0.978e0_f64 * t5639 - 0.22005e1_f64 * t5643 + 0.1467e1_f64 * t5647 + 0.489e0_f64 * t5649 + 0.7335e0_f64 * t5653 + 0.12225e0_f64 * t5657 - 0.61125e-1_f64 * t5661 + t4798 + t4800 - 0.36675e0_f64 * t5664 + 0.489e0_f64 * t4804;
    (t5653, t5655, t5657, t5659, t5661, t5664, t5667)
}
