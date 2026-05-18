//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 687/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk687<F: Float>(t1055: F, t4818: F, t345: F, t495: F, t839: F, t3579: F, t4798: F, t4800: F, t4802: F, t4804: F, t4809: F, t4812: F, t4814: F, t4817: F) -> (F, F, F, F, F, F) {
    let t4819 = t1055 * t4818;
    let t4820 = t345 * t4819;
    let t4822 = t495 * t839;
    let t4823 = t1055 * t4822;
    let t4824 = t345 * t4823;
    let t4826 = t4798 + t4800 - F::new(0.36675e0) * t4802 + F::new(0.2445e0) * t4804 - t4809 - F::new(0.12225e0) * t4812 - F::new(0.1141e1) * t4814 - t4817 + F::new(0.1467e1) * t4820 + F::new(0.7335e0) * t4824 + t3579;
    (t4819, t4820, t4822, t4823, t4824, t4826)
}
