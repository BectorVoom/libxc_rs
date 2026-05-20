//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1562;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1563;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta528<F: Float>(t12866: F, t58895: F, t6639: F, t17448: F, t21090: F, t12916: F, t24730: F, t5340: F, t12809: F, t24839: F, t21063: F, t5362: F, t17308: F, t20846: F, t24639: F, t3172: F, t3711: F, t13062: F, t24545: F, t1261: F, t24807: F, t17377: F, t20786: F, t24604: F, t5384: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t83758, t83783, t83798, t83812, t83849) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1562::<F>(t12866, t58895, t6639, t17448, t21090, t12916, t24730, t5340, t12809, t24839, t21063, t5362);
        let (t83851, t83860, t83863, t83871, t83891, t83897) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1563::<F>(t17308, t20846, t24639, t3172, t3711, t13062, t24545, t1261, t24807, t17377, t20786, t24604, t5384);
    (t83758, t83783, t83798, t83812, t83849, t83851, t83860, t83863, t83871, t83891, t83897)
}
