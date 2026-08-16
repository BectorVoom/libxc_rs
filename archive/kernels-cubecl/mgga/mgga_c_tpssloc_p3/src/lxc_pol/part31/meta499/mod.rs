//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta499<F: Float>(t1339: F, t28100: F, t22827: F, t22833: F, t6396: F, t1842: F, t26337: F, t22635: F, t22633: F, t1825: F, t26421: F, t6976: F) -> (F, F, F, F, F, F, F, F) {
        let (t28101, t28102, t28104, t28116, t28117, t28118, t28130, t28131) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1695::<F>(t1339, t28100, t22827, t22833, t6396, t1842, t26337, t22635, t22633, t1825, t26421, t6976);
    (t28101, t28102, t28104, t28116, t28117, t28118, t28130, t28131)
}
