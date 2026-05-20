//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta197 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta197<F: Float>(t10022: F, t786: F, t3923: F, t675: F, t268: F, t4003: F, t2435: F, t4093: F, t4083: F, t9303: F, t4066: F, t545: F) -> (F, F, F, F, F, F, F) {
        let (t10023, t10024, t10026, t10027, t10032, t10035, t10039) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk962::<F>(t10022, t786, t3923, t675, t268, t4003, t2435, t4093, t4083, t9303, t4066, t545);
    (t10023, t10024, t10026, t10027, t10032, t10035, t10039)
}
