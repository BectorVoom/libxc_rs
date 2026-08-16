//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 845/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk845<F: Float>(t4750: F, t4651: F, t4753: F, t4663: F, t6075: F, t4783: F, t4789: F, t4798: F, t4802: F, t4746: F, t4755: F, t4779: F, t4785: F, t4791: F, t4794: F, t4796: F, t6845: F) -> F {
    let t16349 = F::cast_from(0.4155781415850207192e3_f64) * t4750;
    let t16350 = F::cast_from(0.13780452414814814815e-1_f64) * t4651;
    let t16351 = F::cast_from(144.0_f64) * t4753;
    let t16353 = F::cast_from(0.20690005882282467367e4_f64) * t4663;
    let t16354 = F::cast_from(0.18960024086108224108e1_f64) * t6075;
    let t16356 = F::cast_from(0.41015588084031179722e4_f64) * t4783;
    let t16358 = F::cast_from(0.23392893589820816284e1_f64) * t4789;
    let t16362 = F::cast_from(48.0_f64) * t4798;
    let t16363 = F::cast_from(0.2077890707925103596e3_f64) * t4802;
    let t16364 = F::cast_from(12.0_f64) * t4746 + F::cast_from(72.0_f64) * t6845 + t16349 + t16350 + t16351 + F::cast_from(144.0_f64) * t4755 + t16353 - t16354 + F::cast_from(0.79007158810260824916e-1_f64) * t4779 - t16356 - F::cast_from(0.70178680769462448852e1_f64) * t4785 - t16358 - F::cast_from(0.2077890707925103596e3_f64) * t4791 - F::cast_from(0.70178680769462448852e1_f64) * t4794 + F::cast_from(0.14035736153892489771e2_f64) * t4796 - t16362 - t16363;
    t16364
}
