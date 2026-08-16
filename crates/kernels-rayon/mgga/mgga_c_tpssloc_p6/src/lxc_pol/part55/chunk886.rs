//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 886/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk886(t22690: f64, t6638: f64, t23171: f64, t206: f64, t268: f64, t6559: f64) -> (f64, f64) {
    let t23172 = t22690 * t6638;
    let t23173 = t23171 * t23172;
    let t23174 = 0.82246703342411321824e-2_f64 * t23173;
    let t23185 = t6559 * t206 * t268;
    (t23174, t23185)
}
