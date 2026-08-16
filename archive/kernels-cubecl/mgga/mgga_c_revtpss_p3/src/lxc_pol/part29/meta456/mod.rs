//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1701;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1702;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta456<F: Float>(t1923: F, t26169: F, t2047: F, t25146: F, t10309: F, t7342: F, t38: F, t624: F, t2247: F, t6960: F, t25163: F, t6963: F, t7349: F, t10301: F, t6954: F, t239: F, t72: F, t1927: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26170, t26172, t26175, t26178, t26179) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1701::<F>(t1923, t26169, t2047, t25146, t10309, t7342, t38, t624, t2247);
        let (t26180, t26182, t26185, t26187, t26190, t26204, t26205) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1702::<F>(t26179, t6960, t2047, t25163, t6963, t7349, t10301, t7342, t6954, t239, t72, t1927);
    (t26170, t26172, t26175, t26178, t26179, t26180, t26182, t26185, t26187, t26190, t26204, t26205)
}
