//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta508 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1704;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta508<F: Float>(t28321: F, t6646: F, t1888: F, t5544: F, t6638: F, t6637: F, t6552: F, t1894: F, t5631: F, t214: F, t1880: F, t1510: F, t25249: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28322, t28323, t28329, t28330, t28331, t28333, t28334, t28335, t28337) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1704::<F>(t28321, t6646, t1888, t5544, t6638, t6637, t6552, t1894, t5631, t214, t1880, t1510, t25249);
    (t28322, t28323, t28329, t28330, t28331, t28333, t28334, t28335, t28337)
}
