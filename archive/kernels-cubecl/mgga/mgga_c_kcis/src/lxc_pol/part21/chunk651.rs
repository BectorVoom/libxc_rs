//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 651/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk651<F: Float>(t5175: F, t5177: F, t3436: F, t380: F, t251: F, t3346: F, t4813: F) -> (F, F, F, F) {
    let t5178 = t5175 * t5177;
    let t5180 = t380 * t3436;
    let t5181 = t251 * t3346;
    let t5182 = t5181 * t4813;
    (t5178, t5180, t5181, t5182)
}
