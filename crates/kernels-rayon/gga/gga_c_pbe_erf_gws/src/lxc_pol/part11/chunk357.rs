//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 357/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk357(t156: f64, t472: f64, t1447: f64, t285: f64, t545: f64, t762: f64, t147: f64, t39: f64) -> (f64, f64, f64, f64) {
    let t1448 = t156 * t472;
    let t1449 = t1447 * t1448;
    let t1450 = 0.10843580882781524214e-1_f64 * t1449;
    let t1463 = 0.58113483035773838734e-3_f64 * t762 * t545 * t285;
    let t1464 = t39 * t147;
    (t1448, t1450, t1463, t1464)
}
