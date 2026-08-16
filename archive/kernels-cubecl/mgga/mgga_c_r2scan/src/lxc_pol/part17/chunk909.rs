//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 909/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk909<F: Float>(t4839: F, t4842: F, t4845: F, t5020: F, t7025: F, t7036: F, t8641: F, t8642: F, t8643: F, t9580: F, t9585: F, t9588: F) -> F {
    let t9802 = -t4839 - t8641 + t5020 + t9580 - t4842 + t7025 - t8642 - t8643 + t7036 + t4845 - t9585 - t9588;
    t9802
}
