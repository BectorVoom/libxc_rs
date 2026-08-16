//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2102;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta623<F: Float>(t39063: F, t6489: F, t22573: F, t6875: F, t22947: F, t532: F, t111: F, t22558: F, t7002: F, t112: F, t23862: F, t1395: F, t7020: F, t26555: F, t576: F, t1858: F, t2029: F, t5363: F, t1851: F, t16507: F, t16546: F, t1852: F, t2023: F, t23863: F, t23901: F, t3946: F, t5381: F, t7003: F, t7759: F, t80593: F, t80597: F) -> (F, F, F, F, F, F, F) {
        let (t83830, t83886, t83929, t83935, t83980, t84004, t84024) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2102::<F>(t39063, t6489, t22573, t6875, t22947, t532, t111, t22558, t7002, t112, t23862, t1395, t7020);
        let t86580 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2103::<F>(t26555, t576, t1858, t7002, t2029, t5363, t1851, t7020, t16507, t16546, t1852, t2023, t23863, t23901, t3946, t5381, t7003, t7759, t80593, t80597, t84024);
    (t83830, t83886, t83929, t83935, t83980, t84004, t86580)
}
