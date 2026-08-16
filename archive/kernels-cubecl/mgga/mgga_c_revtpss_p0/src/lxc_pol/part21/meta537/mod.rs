//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2197;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta537<F: Float>(t16831: F, t448: F, t300: F, t1130: F, t5060: F, t1151: F, t3428: F, t5063: F, t1719: F, t3432: F, t3436: F, t12238: F, t1733: F) -> (F, F, F, F, F, F, F, F) {
        let (t16832, t16834, t16835, t16837, t16839, t16840, t16842, t16844) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2197::<F>(t16831, t448, t300, t1130, t5060, t1151, t3428, t5063, t1719, t3432, t3436, t12238, t1733);
    (t16832, t16834, t16835, t16837, t16839, t16840, t16842, t16844)
}
