//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta549<F: Float>(t2035: F, t29506: F, t5920: F, t94: F, t1937: F, t7732: F, t7735: F, t21663: F, t38: F, t5868: F, t76: F, t1470: F, t4173: F) -> (F, F, F, F, F, F, F) {
        let (t29507, t29508, t29510, t29512, t29513, t29532, t29538) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1933::<F>(t2035, t29506, t5920, t94, t1937, t7732, t7735, t21663, t38, t5868, t76, t1470, t4173);
    (t29507, t29508, t29510, t29512, t29513, t29532, t29538)
}
