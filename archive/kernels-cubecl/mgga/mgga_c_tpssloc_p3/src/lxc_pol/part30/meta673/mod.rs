//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta673 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta673<F: Float>(t87779: F, t87898: F, t87915: F, t90503: F, t90551: F, t90582: F, t90642: F, t90659: F, t90663: F, t90837: F, t90868: F, t90900: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t92863, t92954, t92961, t93335, t93368, t93387, t93438, t93445, t93446, t93517, t93538, t93563) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2102::<F>(t87779, t87898, t87915, t90503, t90551, t90582, t90642, t90659, t90663, t90837, t90868, t90900);
    (t92863, t92954, t92961, t93335, t93368, t93387, t93438, t93445, t93446, t93517, t93538, t93563)
}
