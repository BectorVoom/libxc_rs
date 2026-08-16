//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1210;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta401<F: Float>(t10189: F, t5842: F, t5836: F, t5838: F, t698: F, t973: F, t5844: F, t4509: F, t10224: F, t5824: F, t2986: F, t4514: F, t48019: F) -> (F, F, F, F, F, F, F, F) {
        let (t61189, t61250, t61310, t61313, t61322, t61365, t61408, t61489) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1210::<F>(t10189, t5842, t5836, t5838, t698, t973, t5844, t4509, t10224, t5824, t2986, t4514, t48019);
    (t61189, t61250, t61310, t61313, t61322, t61365, t61408, t61489)
}
