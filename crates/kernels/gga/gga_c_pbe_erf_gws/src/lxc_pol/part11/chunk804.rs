//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 804/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk804<F: Float>(t4498: F, t4502: F, t4505: F, t4512: F, t4538: F, t4541: F, t4744: F, t4750: F, t4651: F, t4753: F, t4663: F, t6075: F, t4783: F, t4789: F, t4798: F, t4802: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16334 = 48.0 * t4498;
    let t16335 = 0.19298189186581325787e3 * t4502;
    let t16336 = 24.0 * t4505;
    let t16337 = 0.38596378373162651572e3 * t4512;
    let t16338 = 4.0 * t4538;
    let t16340 = 24.0 * t4541;
    let t16345 = 4.0 * t4744;
    let t16349 = 0.4155781415850207192e3 * t4750;
    let t16350 = 0.13780452414814814815e-1 * t4651;
    let t16351 = 144.0 * t4753;
    let t16353 = 0.20690005882282467367e4 * t4663;
    let t16354 = 0.18960024086108224108e1 * t6075;
    let t16356 = 0.41015588084031179722e4 * t4783;
    let t16358 = 0.23392893589820816284e1 * t4789;
    let t16362 = 48.0 * t4798;
    let t16363 = 0.2077890707925103596e3 * t4802;
    (t16334, t16335, t16336, t16337, t16338, t16340, t16345, t16349, t16350, t16351, t16353, t16354, t16356, t16358, t16362, t16363)
}
