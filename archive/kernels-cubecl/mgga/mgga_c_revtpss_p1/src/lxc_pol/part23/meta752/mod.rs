//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta752 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2541;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta752<F: Float>(t52035: F, t11199: F, t1646: F, t378: F, t11120: F, t1695: F, t11200: F, t1678: F, t3056: F, t4742: F, t379: F, t51973: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t52955, t53014, t53015, t53108, t53160, t53166, t53167, t53174, t53243) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2541::<F>(t52035, t11199, t1646, t378, t11120, t1695, t11200, t1678, t3056, t4742, t379, t51973);
    (t52955, t53014, t53015, t53108, t53160, t53166, t53167, t53174, t53243)
}
