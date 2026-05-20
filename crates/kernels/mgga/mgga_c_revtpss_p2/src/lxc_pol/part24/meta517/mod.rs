//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1538;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta517<F: Float>(t1678: F, t19462: F, t1086: F, t23959: F, t23997: F, t3153: F, t3154: F, t6299: F, t12050: F, t357: F, t11631: F, t24042: F, t359: F) -> (F, F, F, F, F, F, F) {
        let (t80173, t80243, t80264, t80277, t80350, t80358, t80396) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1538::<F>(t1678, t19462, t1086, t23959, t23997, t3153, t3154, t6299, t12050, t357, t11631, t24042, t359);
    (t80173, t80243, t80264, t80277, t80350, t80358, t80396)
}
