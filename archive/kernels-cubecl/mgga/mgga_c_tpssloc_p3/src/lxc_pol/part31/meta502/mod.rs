//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1698;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta502<F: Float>(t19660: F, t550: F, t6976: F, t1992: F, t19743: F, t6330: F, t6890: F, t6889: F, t22685: F, t26193: F, t7700: F, t1985: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28167, t28168, t28169, t28181, t28182, t28183, t28191, t28192, t28193, t28195, t28196) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1698::<F>(t19660, t550, t6976, t1992, t19743, t6330, t6890, t6889, t22685, t26193, t7700, t1985);
    (t28167, t28168, t28169, t28181, t28182, t28183, t28191, t28192, t28193, t28195, t28196)
}
