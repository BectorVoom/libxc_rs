//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta233<F: Float>(t3403: F, t6105: F, t1164: F, t338: F, t5416: F, t3441: F, t5392: F, t3440: F, t4904: F, t4919: F, t3455: F, t1177: F) -> (F, F, F, F, F, F, F, F) {
        let (t6106, t6108, t6109, t6119, t6120, t6123, t6126, t6127) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1060::<F>(t3403, t6105, t1164, t338, t5416, t3441, t5392, t3440, t4904, t4919, t3455, t1177);
    (t6106, t6108, t6109, t6119, t6120, t6123, t6126, t6127)
}
