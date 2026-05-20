//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta602<F: Float>(t45972: F, t7565: F, t45963: F, t10309: F, t26754: F, t12627: F, t2142: F, t12640: F, t11239: F, t1269: F, t1276: F, t2148: F) -> (F, F, F, F, F, F) {
        let (t96804, t96824, t96827, t96861, t96866, t96883) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2026::<F>(t45972, t7565, t45963, t10309, t26754, t12627, t2142, t12640, t11239, t1269, t1276, t2148);
    (t96804, t96824, t96827, t96861, t96866, t96883)
}
