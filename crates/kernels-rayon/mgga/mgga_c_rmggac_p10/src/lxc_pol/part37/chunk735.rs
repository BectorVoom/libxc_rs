//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 735/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk735(t68794: f64, t68801: f64, t14668: f64, t16156: f64, t14385: f64, t34884: f64, t14672: f64, t68950: f64, t118: f64, t2001: f64, t498: f64, t699: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t71043 = 0.162600798888400151e-2_f64 * t68794;
    let t71046 = 0.10492326631435615411e0_f64 * t68801;
    let t71097 = t16156 * t14668;
    let t71109 = t34884 * t14385;
    let t71112 = t16156 * t14672;
    let t71151 = 0.51300288795035171252e-6_f64 * t68950;
    let t71154 = t2001 * t118 * t699 * t498;
    (t71043, t71046, t71097, t71109, t71112, t71151, t71154)
}
