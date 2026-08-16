//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1873;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta584<F: Float>(t13450: F, t1888: F, t6646: F, t23110: F, t23185: F, t4292: F, t25288: F, t81591: F, t234: F, t4265: F, t6552: F, t6637: F, t776: F, t25237: F, t23168: F, t25307: F, t10007: F, t22986: F, t4282: F, t25287: F, t81651: F, t13401: F, t22996: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t87578, t87581, t87583, t87589) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1873::<F>(t13450, t1888, t6646, t23110, t23185, t4292, t25288, t81591, t234, t4265, t6552, t6637, t776);
        let (t87601, t87603, t87609, t87612, t87615) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1874::<F>(t23110, t23185, t25237, t23168, t25307, t10007, t22986, t4282, t6646, t25287, t81651, t13401, t1888, t22996);
    (t87578, t87581, t87583, t87589, t87601, t87603, t87609, t87612, t87615)
}
