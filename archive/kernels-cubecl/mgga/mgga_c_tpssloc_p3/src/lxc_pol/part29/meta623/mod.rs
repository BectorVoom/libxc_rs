//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2065;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta623<F: Float>(t11651: F, t24733: F, t11797: F, t7345: F, t11835: F, t7310: F, t11647: F, t2141: F, t1184: F, t607: F, t24682: F, t460: F) -> (F, F, F, F, F, F) {
        let (t86174, t86176, t86184, t86191, t86192, t86194) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2065::<F>(t11651, t24733, t11797, t7345, t11835, t7310, t11647, t2141, t1184, t607, t24682, t460);
    (t86174, t86176, t86184, t86191, t86192, t86194)
}
