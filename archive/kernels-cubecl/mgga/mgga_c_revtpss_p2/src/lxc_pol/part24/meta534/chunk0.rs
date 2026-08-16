//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1573/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1573<F: Float>(t22837: F, t9962: F, t22860: F, t47194: F, t22849: F, t3957: F, t13790: F, t22020: F, t2661: F, t9934: F, t177: F, t22789: F, t762: F) -> (F, F, F, F, F) {
    let t85839 = t9962 * t22837;
    let t85865 = t47194 * t22860;
    let t85873 = t3957 * t22849;
    let t85885 = t2661 * t9934 * t22020 * t13790;
    let t85895 = t22789 * t177 * t762;
    (t85839, t85865, t85873, t85885, t85895)
}
