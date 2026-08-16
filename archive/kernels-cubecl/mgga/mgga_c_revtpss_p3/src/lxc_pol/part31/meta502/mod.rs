//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1822;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta502<F: Float>(t30: F, t892: F, t4433: F, t18875: F, t25207: F, t1544: F, t605: F, t4343: F, t1949: F, t4533: F, t7071: F, t689: F, t7774: F) -> (F, F, F, F, F, F, F, F) {
        let (t27159, t27160, t27166, t27169, t27173, t27182, t27183, t27186) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1822::<F>(t30, t892, t4433, t18875, t25207, t1544, t605, t4343, t1949, t4533, t7071, t689, t7774);
    (t27159, t27160, t27166, t27169, t27173, t27182, t27183, t27186)
}
