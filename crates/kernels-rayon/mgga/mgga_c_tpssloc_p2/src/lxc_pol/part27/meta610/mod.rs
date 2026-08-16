//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta610(t23473: f64, t82892: f64, t23509: f64, t25651: f64, t1015: f64, t23520: f64, t23563: f64, t25650: f64, t3082: f64, t6750: f64, t607: f64, t984: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t82893, t82895, t82897, t82911, t82914, t82916) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2083(t23473, t82892, t23509, t25651, t1015, t23520, t23563, t25650, t3082, t6750, t607, t984);
    (t82893, t82895, t82897, t82911, t82914, t82916)
}
