//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 600/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk600<F: Float>(t1055: F, t5651: F, t345: F, t1713: F, t355: F, t721: F, t3115: F, t1734: F, t1060: F, t346: F, t5506: F, t4798: F, t4800: F, t4804: F, t5639: F, t5643: F, t5647: F, t5649: F) -> (F, F, F, F, F, F, F) {
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
    let t5667 = -F::new(0.978e0) * t5639 - F::new(0.22005e1) * t5643 + F::new(0.1467e1) * t5647 + F::new(0.489e0) * t5649 + F::new(0.7335e0) * t5653 + F::new(0.12225e0) * t5657 - F::new(0.61125e-1) * t5661 + t4798 + t4800 - F::new(0.36675e0) * t5664 + F::new(0.489e0) * t4804;
    (t5653, t5655, t5657, t5659, t5661, t5664, t5667)
}
