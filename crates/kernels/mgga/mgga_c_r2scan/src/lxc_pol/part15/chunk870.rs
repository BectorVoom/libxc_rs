//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 870/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk870<F: Float>(t4842: F, t4845: F, t5020: F, t6010: F, t6012: F, t7020: F, t7021: F, t7025: F, t7031: F, t7033: F, t7036: F, t6026: F, t7052: F, t7055: F, t7093: F, t7095: F, t7097: F, t7098: F, t7101: F, t7104: F, t7133: F, t7136: F, t765: F) -> (F, F) {
    let t7884 = t7020 - t7021 + t5020 + t6010 - F::new(0.1143056e0) * t6012 - t4842 - t7025 - t7031 - t7033 + t7036 + t4845;
    let t7895 = F::new(0.675260332e-1) * t765 * t7098 + F::new(0.1350520664e0) * t765 * t7101 + F::new(0.675260332e-1) * t765 * t7104 + F::new(0.675260332e-1) * t765 * t7133 + F::new(0.1350520664e0) * t765 * t7136 + t7052 - t7055 - t6026 - t7093 - t7095 + t7097;
    (t7884, t7895)
}
