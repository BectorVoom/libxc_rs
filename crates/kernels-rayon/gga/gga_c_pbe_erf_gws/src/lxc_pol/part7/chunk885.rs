//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 885/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk885(t16853: f64, t16684: f64, t2559: f64, t587: f64, t1820: f64, t4952: f64, t562: f64, t7435: f64, t1416: f64, t1815: f64, t4896: f64, t639: f64) -> (f64, f64, f64, f64) {
    let t16854 = 32.0_f64 / 15.0_f64 * t16853;
    let t16857 = 16.0_f64 / 27.0_f64 * t587 * t2559 * t16684;
    let t16861 = 256.0_f64 / 81.0_f64 * t1820 * t7435 * t4952 * t562;
    let t16865 = 8.0_f64 / 15.0_f64 * t639 * t1815 * t4896 * t1416;
    (t16854, t16857, t16861, t16865)
}
