//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 849/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk849<F: Float>(t2769: F, t2774: F, t5727: F, t5736: F, t5739: F, t5891: F, t5895: F, t5898: F, t7761: F, t7807: F, t7810: F, t7813: F, t7817: F, t951: F) -> F {
    let t8984 = -F::cast_from(0.1350520664e0_f64) * t2774 * t2769 - F::cast_from(0.1350520664e0_f64) * t951 * t7761 + t5727 - t5736 - t5739 - t5891 - t5895 - t7807 - F::cast_from(32.0_f64) * t5898 - t7810 - t7813 - F::cast_from(0.20010214504933333333e-2_f64) * t7817;
    t8984
}
