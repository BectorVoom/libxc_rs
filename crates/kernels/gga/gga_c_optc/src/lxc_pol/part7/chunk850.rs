//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 850/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk850<F: Float>(t8587: F, t8589: F, t8591: F, t8593: F, t8595: F, t8598: F, t8601: F, t8603: F, t8606: F, t8609: F, t8613: F, t8618: F, t8622: F, t8625: F, t1052: F, t1782: F) -> (F, F) {
    let t8627 = -0.82156666666666666667e-1 * t8587 - 0.39862222222222222223e0 * t8589 + 0.29896666666666666667e0 * t8591 + 0.19931111111111111111e0 * t8593 - 0.32862666666666666666e0 * t8595 + 0.16431333333333333333e0 * t8598 - 0.49293999999999999999e0 * t8601 - 0.59793333333333333333e0 * t8603 + 0.11958666666666666667e1 * t8606 - 0.17938e1 * t8609 - 0.76790625e-1 * t8613 + 0.142419375e1 * t8618 - 0.33218518518518518518e0 * t8622 - 0.29896666666666666667e0 * t8625;
    let t8628 = t1782 * t1052;
    (t8627, t8628)
}
