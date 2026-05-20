//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1917;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta588<F: Float>(t2435: F, t8011: F, t25431: F, t2439: F, t93170: F, t28347: F, t686: F, t72: F, t25387: F, t102980: F, t93190: F, t10073: F, t26554: F, t27198: F) -> (F, F, F, F, F, F, F, F) {
        let (t102993, t102994, t103000, t103001, t103005, t103007, t103009, t103017) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1917::<F>(t2435, t8011, t25431, t2439, t93170, t28347, t686, t72, t25387, t102980, t93190, t10073, t26554, t27198);
    (t102993, t102994, t103000, t103001, t103005, t103007, t103009, t103017)
}
