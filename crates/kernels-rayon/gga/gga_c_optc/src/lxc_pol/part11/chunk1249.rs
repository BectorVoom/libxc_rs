//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1249/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1249(t4727: f64, t22497: f64, t22562: f64, t22578: f64, t22581: f64, t22593: f64, t22697: f64, t22703: f64, t22708: f64, t22711: f64, t48162: f64, t56294: f64) -> (f64, f64) {
    let t56654 = t4727 * t4727;
    let t56661 = -t22497 + t22562 + t22578 + t22581 - t22593 - t56294 - 14.0_f64 * t48162 + t22697 + t22703 + t22708 - t22711;
    (t56654, t56661)
}
