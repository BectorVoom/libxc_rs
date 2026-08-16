//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1297;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1298;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta270<F: Float>(t2020: F, t7685: F, t1390: F, t1799: F, t6878: F, t1983: F, t6890: F, t6889: F, t6888: F, t1834: F, t225: F, t567: F, t214: F, t1985: F, t1842: F, t6906: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7686, t7687, t7688, t7690, t7691) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1297::<F>(t2020, t7685, t1390, t1799, t6878, t1983, t6890);
        let (t7692, t7693, t7696, t7697, t7698, t7700) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1298::<F>(t6889, t7691, t6888, t1834, t225, t567, t214, t1985, t1842, t6906);
    (t7686, t7687, t7688, t7690, t7691, t7692, t7693, t7696, t7697, t7698, t7700)
}
