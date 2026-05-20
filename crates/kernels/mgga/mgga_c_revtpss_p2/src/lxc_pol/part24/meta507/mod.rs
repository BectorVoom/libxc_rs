//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1517;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1518;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta507<F: Float>(t2723: F, t2782: F, t4503: F, t76169: F, t14568: F, t18726: F, t10871: F, t14545: F, t231: F, t2783: F, t76127: F, t23359: F, t822: F, t213: F, t262: F, t5966: F, t23148: F, t23421: F, t2411: F, t11064: F, t23429: F, t892: F, t23478: F, t689: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t77177, t77183, t77191, t77197, t77225) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1517::<F>(t2723, t2782, t4503, t76169, t14568, t18726, t10871, t14545, t231, t2783, t76127, t23359, t822);
        let (t77316, t77333, t77341, t77357, t77373, t77460, t77499) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1518::<F>(t213, t23359, t262, t5966, t23148, t23421, t2411, t11064, t23429, t892, t23478, t689);
    (t77177, t77183, t77191, t77197, t77225, t77316, t77333, t77341, t77357, t77373, t77460, t77499)
}
