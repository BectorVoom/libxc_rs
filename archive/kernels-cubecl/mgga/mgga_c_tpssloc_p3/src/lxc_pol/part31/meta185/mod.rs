//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta185 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta185<F: Float>(t1229: F, t3247: F, t3961: F, t4582: F, t1734: F, t486: F, t1215: F, t3508: F, t1216: F, t3242: F, t3584: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4972, t4973, t4974, t4977, t4978, t4979, t4980, t4983, t4984, t4987, t4988, t4989) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk841::<F>(t1229, t3247, t3961, t4582, t1734, t486, t1215, t3508, t1216, t3242, t3584);
    (t4972, t4973, t4974, t4977, t4978, t4979, t4980, t4983, t4984, t4987, t4988, t4989)
}
