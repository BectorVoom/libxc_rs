//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2063;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta649<F: Float>(t23384: F, t25518: F, t10277: F, t381: F, t225: F, t25608: F, t25714: F, t7604: F, t82573: F, t25718: F, t23665: F, t25541: F) -> (F, F, F, F, F, F, F) {
        let (t89057, t89071, t89076, t89094, t89104, t89151, t89156) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2063::<F>(t23384, t25518, t10277, t381, t225, t25608, t25714, t7604, t82573, t25718, t23665, t25541);
    (t89057, t89071, t89076, t89094, t89104, t89151, t89156)
}
