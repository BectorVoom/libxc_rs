//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 880/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk880(t4744: f64, t4750: f64, t4651: f64, t4753: f64, t4663: f64, t6075: f64, t4783: f64, t4789: f64, t4798: f64, t4802: f64, t4806: f64, t4814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16345 = 4.0_f64 * t4744;
    let t16349 = 0.4155781415850207192e3_f64 * t4750;
    let t16350 = 0.13780452414814814815e-1_f64 * t4651;
    let t16351 = 144.0_f64 * t4753;
    let t16353 = 0.20690005882282467367e4_f64 * t4663;
    let t16354 = 0.18960024086108224108e1_f64 * t6075;
    let t16356 = 0.41015588084031179722e4_f64 * t4783;
    let t16358 = 0.23392893589820816284e1_f64 * t4789;
    let t16362 = 48.0_f64 * t4798;
    let t16363 = 0.2077890707925103596e3_f64 * t4802;
    let t16366 = 0.14035736153892489771e2_f64 * t4806;
    let t16368 = 0.22787712934626154593e-2_f64 * t4814;
    (t16345, t16349, t16350, t16351, t16353, t16354, t16356, t16358, t16362, t16363, t16366, t16368)
}
