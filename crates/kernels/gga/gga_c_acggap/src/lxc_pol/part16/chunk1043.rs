//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1043/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1043<F: Float>(t34286: F, t34293: F, t34315: F, t34317: F, t34392: F, t34394: F, t34396: F, t34400: F, t34433: F, t34453: F, t34468: F, t34476: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37009 = F::cast_from(0.90035438047946447644e-2_f64) * t34286;
    let t37012 = F::cast_from(0.32012600194825403606e-1_f64) * t34293;
    let t37021 = F::cast_from(0.17149607247227894789e-2_f64) * t34315;
    let t37022 = F::cast_from(0.25724410870841842184e-2_f64) * t34317;
    let t37066 = F::cast_from(0.26147916666666666667e0_f64) * t34392;
    let t37067 = F::cast_from(0.42874018118069736972e-3_f64) * t34394;
    let t37068 = F::cast_from(0.16006300097412701803e-1_f64) * t34396;
    let t37070 = F::cast_from(0.34299214494455789578e-2_f64) * t34400;
    let t37093 = F::cast_from(77.0_f64) / F::cast_from(864.0_f64) * t34433;
    let t37105 = F::cast_from(0.10718504529517434243e-2_f64) * t34453;
    let t37112 = F::cast_from(0.90035438047946447644e-2_f64) * t34468;
    let t37114 = F::cast_from(0.18868855373762491241e-2_f64) * t34476;
    (t37009, t37012, t37021, t37022, t37066, t37067, t37068, t37070, t37093, t37105, t37112, t37114)
}
