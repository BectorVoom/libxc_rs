//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1743;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta461<F: Float>(t23110: F, t6648: F, t23185: F, t225: F, t2717: F, t252: F, t794: F, t6555: F, t23164: F, t6572: F, t6562: F, t212: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t23186, t23187, t23195, t23204) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1743::<F>(t23110, t6648, t23185, t225, t2717, t252, t794);
        let (t23205, t23206, t23208, t23209, t23228) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1744::<F>(t23204, t6555, t23164, t6572, t6562, t212, t252);
    (t23186, t23187, t23195, t23204, t23205, t23206, t23208, t23209, t23228)
}
