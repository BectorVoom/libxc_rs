//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1683;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta472<F: Float>(t4292: F, t6646: F, t1888: F, t2647: F, t4282: F, t22986: F, t6547: F, t7529: F, t25249: F, t829: F, t22996: F, t4283: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t25284, t25285, t25287, t25288, t25289, t25293, t25299, t25300, t25301, t25303) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1683::<F>(t4292, t6646, t1888, t2647, t4282, t22986, t6547, t7529, t25249, t829, t22996, t4283);
    (t25284, t25285, t25287, t25288, t25289, t25293, t25299, t25300, t25301, t25303)
}
