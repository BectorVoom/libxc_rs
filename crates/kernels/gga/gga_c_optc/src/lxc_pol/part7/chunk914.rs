//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 914/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk914<F: Float>(t8639: F, t8589: F, t8591: F, t8593: F, t8603: F, t8606: F, t8609: F, t8622: F, t8625: F, t8657: F, t8660: F, t1045: F) -> (F, F) {
    let t8662 = F::new(28.0) / F::new(27.0) * t8639;
    let t8673 = -t8662 - F::new(4.0) / F::new(9.0) * t8589 + F::new(2.0) / F::new(9.0) * t8593 - F::new(2.0) / F::new(3.0) * t8603 + t8591 / F::new(3.0) - F::new(10.0) / F::new(27.0) * t8622 + F::new(4.0) / F::new(3.0) * t8606 - F::new(2.0) / F::new(3.0) * t8657 - F::new(2.0) * t8609 + F::new(2.0) * t8660 - t8625 / F::new(3.0);
    let t8674 = t1045 * t8673;
    (t8673, t8674)
}
