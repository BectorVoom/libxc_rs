//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1915;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta516<F: Float>(t1544: F, t1583: F, t18875: F, t1940: F, t1963: F, t198: F, t207: F, t2403: F, t25440: F, t25445: F, t27363: F, t27368: F, t27375: F, t27384: F, t4343: F, t4433: F, t4537: F, t4541: F, t7087: F, t7091: F, t775: F, t7783: F, t890: F, t892: F, t33: F, t25759: F, t1113: F, t11064: F) -> (F, F, F, F, F, F, F, F) {
        let t27754 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1915::<F>(t1544, t1583, t18875, t1940, t1963, t198, t207, t2403, t25440, t25445, t27363, t27368, t27375, t27384, t4343, t4433, t4537, t4541, t7087, t7091, t775, t7783, t890, t892);
        let (t27763, t27764, t27770, t27773, t27777, t27793, t27799) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1916::<F>(t33, t892, t4433, t18875, t25759, t1113, t1544, t4343, t27375, t11064);
    (t27754, t27763, t27764, t27770, t27773, t27777, t27793, t27799)
}
