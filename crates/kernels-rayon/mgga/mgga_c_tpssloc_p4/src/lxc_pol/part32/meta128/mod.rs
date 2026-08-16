//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta128 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk733;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta128(t1097: f64, t409: f64) -> (f64, f64, f64) {
        let (t3311, t3312, t3313) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk733(t1097, t409);
    (t3311, t3312, t3313)
}
