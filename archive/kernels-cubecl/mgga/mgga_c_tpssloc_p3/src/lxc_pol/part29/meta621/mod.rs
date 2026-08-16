//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2063;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta621<F: Float>(t1089: F, t1235: F, t7327: F, t11786: F, t7345: F, t24716: F, t3572: F, t24736: F, t3523: F, t11813: F, t7338: F, t3566: F, t7344: F) -> (F, F, F, F, F, F) {
        let (t86116, t86120, t86122, t86124, t86126, t86129) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2063::<F>(t1089, t1235, t7327, t11786, t7345, t24716, t3572, t24736, t3523, t11813, t7338, t3566, t7344);
    (t86116, t86120, t86122, t86124, t86126, t86129)
}
