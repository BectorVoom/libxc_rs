//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1049/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1049<F: Float>(t10586: F, t10589: F, t10592: F, t10594: F, t10596: F, t10598: F, t10602: F, t10604: F, t10607: F, t10609: F, t10611: F, t10614: F, t9524: F, t9542: F) -> F {
    let t11093 = -t10586 - t9524 - t10589 + t10592 - t10594 - t10596 - t10598 + t10602 - t10604 + t9542 + t10607 + t10609 - t10611 + t10614;
    t11093
}
