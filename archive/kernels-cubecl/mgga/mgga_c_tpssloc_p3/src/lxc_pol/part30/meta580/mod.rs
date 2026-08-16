//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1959;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta580<F: Float>(t3701: F, t6995: F, t1307: F, t2018: F, t1862: F, t31: F, t1458: F, t1868: F, t7752: F, t576: F, t1409: F, t1390: F) -> (F, F, F, F, F, F, F) {
        let (t31035, t31299, t33085, t33136, t33185, t33567, t34999) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1959::<F>(t3701, t6995, t1307, t2018, t1862, t31, t1458, t1868, t7752, t576, t1409, t1390);
    (t31035, t31299, t33085, t33136, t33185, t33567, t34999)
}
