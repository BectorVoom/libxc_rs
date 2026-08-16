//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1869;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta532(t24601: f64, t27437: f64, t24590: f64, t8002: f64, t3247: f64, t497: f64, t3961: f64, t24574: f64, t8067: f64, t1184: f64, t1715: f64, t24745: f64, t7363: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27438, t27441, t27444, t27445, t27446, t27451, t27453, t27454) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1869(t24601, t27437, t24590, t8002, t3247, t497, t3961, t24574, t8067, t1184, t1715, t24745, t7363);
    (t27438, t27441, t27444, t27445, t27446, t27451, t27453, t27454)
}
