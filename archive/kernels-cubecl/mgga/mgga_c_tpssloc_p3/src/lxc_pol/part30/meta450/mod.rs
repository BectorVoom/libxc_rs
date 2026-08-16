//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta450 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1711;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1712;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta450<F: Float>(t1878: F, t557: F, t556: F, t598: F, t213: F, t281: F, t6931: F, t1351: F, t22705: F, t236: F, t550: F, t2003: F, t3862: F, t1358: F, t6940: F, t1887: F, t22715: F, t534: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22839, t22842, t22843, t22844, t22845, t22852) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1711::<F>(t1878, t557, t556, t598, t213, t281, t6931);
        let (t22855, t22856, t22859, t22860, t22863) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1712::<F>(t1351, t22705, t236, t550, t22852, t2003, t3862, t1358, t6940, t1887, t22715, t534);
    (t22839, t22842, t22843, t22844, t22845, t22852, t22855, t22856, t22859, t22860, t22863)
}
