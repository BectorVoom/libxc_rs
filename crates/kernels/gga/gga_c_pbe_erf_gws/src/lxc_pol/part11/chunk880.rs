//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 880/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk880<F: Float>(t4744: F, t4750: F, t4651: F, t4753: F, t4663: F, t6075: F, t4783: F, t4789: F, t4798: F, t4802: F, t4806: F, t4814: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16345 = F::new(4.0) * t4744;
    let t16349 = F::cast_from(0.4155781415850207192e3_f64) * t4750;
    let t16350 = F::cast_from(0.13780452414814814815e-1_f64) * t4651;
    let t16351 = F::new(144.0) * t4753;
    let t16353 = F::cast_from(0.20690005882282467367e4_f64) * t4663;
    let t16354 = F::cast_from(0.18960024086108224108e1_f64) * t6075;
    let t16356 = F::cast_from(0.41015588084031179722e4_f64) * t4783;
    let t16358 = F::cast_from(0.23392893589820816284e1_f64) * t4789;
    let t16362 = F::new(48.0) * t4798;
    let t16363 = F::cast_from(0.2077890707925103596e3_f64) * t4802;
    let t16366 = F::cast_from(0.14035736153892489771e2_f64) * t4806;
    let t16368 = F::cast_from(0.22787712934626154593e-2_f64) * t4814;
    (t16345, t16349, t16350, t16351, t16353, t16354, t16356, t16358, t16362, t16363, t16366, t16368)
}
