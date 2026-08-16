//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta587<F: Float>(t102972: F, t25431: F, t136: F, t2457: F, t8006: F, t93377: F, t28314: F, t93342: F, t28417: F, t686: F, t72: F, t25375: F) -> (F, F, F, F, F, F) {
        let (t102974, t102980, t102981, t102984, t102986, t102988) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1916::<F>(t102972, t25431, t136, t2457, t8006, t93377, t28314, t93342, t28417, t686, t72, t25375);
    (t102974, t102980, t102981, t102984, t102986, t102988)
}
