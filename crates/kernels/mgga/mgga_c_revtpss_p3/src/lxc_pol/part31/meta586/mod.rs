//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta586<F: Float>(t530: F, t7311: F, t2470: F, t26049: F, t7284: F, t2453: F, t555: F, t25898: F, t136: F, t137: F, t2022: F, t1399: F, t2438: F) -> (F, F, F, F, F, F, F) {
        let (t94345, t94377, t94378, t94382, t94383, t94385, t94386) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2008::<F>(t530, t7311, t2470, t26049, t7284, t2453, t555, t25898, t136, t137, t2022, t1399, t2438);
    (t94345, t94377, t94378, t94382, t94383, t94385, t94386)
}
