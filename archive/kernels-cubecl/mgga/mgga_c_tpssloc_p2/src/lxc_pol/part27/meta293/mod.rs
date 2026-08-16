//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta293 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1351;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1352;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta293<F: Float>(t2563: F, t2610: F, t225: F, t2592: F, t2710: F, t814: F, t252: F, t2678: F, t856: F, t68: F, t2745: F, t870: F, t261: F, t2751: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10038, t10049, t10076, t10097, t10108, t10109, t10110, t10126) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1351::<F>(t2563, t2610, t225, t2592, t2710, t814, t252, t2678, t856, t68, t2745, t870);
        let t10143 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1352::<F>(t261, t2751);
    (t10038, t10049, t10076, t10097, t10108, t10109, t10110, t10126, t10143)
}
