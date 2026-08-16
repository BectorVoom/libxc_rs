//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2046;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta636<F: Float>(t25608: F, t381: F, t13797: F, t1926: F, t221: F, t10216: F, t387: F, t10277: F, t1625: F, t225: F, t344: F, t25796: F, t4547: F) -> (F, F, F, F, F, F) {
        let (t88004, t88022, t88023, t88035, t88050, t88058) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2046::<F>(t25608, t381, t13797, t1926, t221, t10216, t387, t10277, t1625, t225, t344, t25796, t4547);
    (t88004, t88022, t88023, t88035, t88050, t88058)
}
