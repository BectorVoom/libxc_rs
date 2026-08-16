//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta524<F: Float>(t21272: F, t5378: F, t12772: F, t24793: F, t3625: F, t24803: F, t44425: F, t1208: F, t24697: F, t225: F, t480: F, t17438: F, t20846: F) -> (F, F, F, F, F, F, F) {
        let (t83018, t83047, t83067, t83107, t83108, t83109, t83112) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1555::<F>(t21272, t5378, t12772, t24793, t3625, t24803, t44425, t1208, t24697, t225, t480, t17438, t20846);
    (t83018, t83047, t83067, t83107, t83108, t83109, t83112)
}
