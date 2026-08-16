//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1752;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta444<F: Float>(t1351: F, t22705: F, t236: F, t550: F, t22852: F, t2003: F, t3862: F, t1358: F, t6940: F, t1887: F, t22715: F, t534: F, t1995: F, t9223: F, t213: F, t1999: F, t22805: F, t22809: F, t22820: F, t22826: F, t22830: F, t22834: F, t22837: F, t22840: F, t22848: F, t22850: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22855, t22856, t22859, t22860, t22861, t22863) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1752::<F>(t1351, t22705, t236, t550, t22852, t2003, t3862, t1358, t6940, t1887, t22715, t534);
        let (t22864, t22865, t22868, t22869) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1753::<F>(t22863, t1995, t9223, t213, t1999, t22805, t22809, t22820, t22826, t22830, t22834, t22837, t22840, t22848, t22850, t22856, t22859, t22861);
    (t22855, t22856, t22859, t22860, t22863, t22864, t22865, t22868, t22869)
}
