//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2046;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta581<F: Float>(t1035: F, t1983: F, t94014: F, t3057: F, t7135: F, t11200: F, t1976: F, t3063: F, t8521: F, t7143: F, t36870: F, t25625: F) -> (F, F, F, F, F, F, F) {
        let (t94016, t94023, t94026, t94042, t94053, t94063, t94068) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2046::<F>(t1035, t1983, t94014, t3057, t7135, t11200, t1976, t3063, t8521, t7143, t36870, t25625);
    (t94016, t94023, t94026, t94042, t94053, t94063, t94068)
}
