//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 909/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk909<F: Float>(t4827: F, t4992: F, t4996: F, t5000: F, t5004: F, t5008: F, t6798: F, t8634: F, t8636: F, t8638: F, t9566: F, t9569: F, t9576: F) -> F {
    let t9799 = -t9566 + t8634 + t4992 - t9569 - t8636 - t8638 + t6798 - t4996 + t5000 + t5004 + t5008 - t9576 + t4827;
    t9799
}
