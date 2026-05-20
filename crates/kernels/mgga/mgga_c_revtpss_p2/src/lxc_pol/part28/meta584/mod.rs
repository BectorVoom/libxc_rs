//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta584<F: Float>(t136: F, t137: F, t2022: F, t1399: F, t2438: F, t94383: F, t25304: F, t555: F, t25898: F, t25876: F, t25931: F, t25894: F) -> (F, F, F, F, F, F, F) {
        let (t94385, t94388, t94390, t94391, t94392, t94394, t94395) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2049::<F>(t136, t137, t2022, t1399, t2438, t94383, t25304, t555, t25898, t25876, t25931, t25894);
    (t94385, t94388, t94390, t94391, t94392, t94394, t94395)
}
