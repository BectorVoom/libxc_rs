//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta983 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta983<F: Float>(t15390: F, t15421: F, t11294: F, t19318: F, t11528: F, t19321: F, t19324: F, t41883: F, t11289: F, t6142: F, t19128: F, t2869: F) -> (F, F, F, F, F, F) {
        let (t63218, t63220, t63222, t63224, t63226, t63228) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3333::<F>(t15390, t15421, t11294, t19318, t11528, t19321, t19324, t41883, t11289, t6142, t19128, t2869);
    (t63218, t63220, t63222, t63224, t63226, t63228)
}
