//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 531/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk531<F: Float>(t4170: F, t5637: F, t4160: F, t1489: F, t1650: F, t4163: F) -> (F, F, F) {
    let t5638 = t4170 * t5637;
    let t5639 = t4160 * t5638;
    let t5643 = t1650 * t1489;
    let t5644 = t4163 * t5643;
    (t5638, t5639, t5644)
}
