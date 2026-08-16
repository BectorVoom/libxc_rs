//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1353;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta294<F: Float>(t225: F, t3023: F, t1053: F, t68: F, t3021: F, t1887: F, t337: F, t615: F, t134: F, t976: F) -> (F, F, F, F, F, F, F) {
        let (t10160, t10163, t10164, t10165, t10170, t10186, t10189) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1353::<F>(t225, t3023, t1053, t68, t3021, t1887, t337, t615, t134, t976);
    (t10160, t10163, t10164, t10165, t10170, t10186, t10189)
}
