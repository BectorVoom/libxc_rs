//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta638 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2048;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta638(t23328: f64, t6705: f64, t225: f64, t25791: f64, t23384: f64, t25413: f64, t1921: f64, t7577: f64, t25403: f64, t25749: f64, t6698: f64, t7566: f64, t82573: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t88112, t88145, t88152, t88162, t88167, t88182, t88194) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2048(t23328, t6705, t225, t25791, t23384, t25413, t1921, t7577, t25403, t25749, t6698, t7566, t82573);
    (t88112, t88145, t88152, t88162, t88167, t88182, t88194)
}
