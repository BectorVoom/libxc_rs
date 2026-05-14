//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 789/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk789<F: Float>(t4806: F, t4992: F, t4996: F, t6002: F, t7870: F, t7874: F, t7876: F, t7878: F, t8634: F, t8636: F, t8638: F, t4827: F, t4839: F, t4842: F, t5000: F, t5004: F, t5008: F, t5020: F, t6010: F, t6012: F, t7025: F, t8641: F) -> (F, F) {
    let t9051 = -t4806 + t8634 + t4992 - 0.675260332e-1 * t6002 - t8636 - t8638 - 0.1350520664e0 * t7870 - t7874 - t7876 + 0.2701041328e0 * t7878 - t4996;
    let t9055 = t5000 + t5004 + t5008 + t4827 - t4839 - t8641 + t5020 + t6010 - 0.571528e-1 * t6012 - t4842 + t7025;
    (t9051, t9055)
}
