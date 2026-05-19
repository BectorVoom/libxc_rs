//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1207/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1207<F: Float>(t39485: F, t39499: F, t39502: F, t37616: F, t37619: F, t37630: F, t37634: F, t37639: F, t37656: F, t39482: F, t39490: F, t39495: F) -> F {
    let t41405 = F::cast_from(0.93443229163669953711e-1_f64) * t39485;
    let t41414 = F::cast_from(0.46230515946956099004e0_f64) * t39499;
    let t41415 = F::cast_from(0.1536604809351619373e1_f64) * t39502;
    let t41416 = F::cast_from(0.31147743054556651237e-1_f64) * t39482 + t41405 - F::cast_from(0.16951189180550569635e1_f64) * t37616 + F::cast_from(0.23115257973478049502e0_f64) * t37619 + F::cast_from(0.17336443480108537126e0_f64) * t39490 - F::cast_from(0.23804984598836975486e0_f64) * t37630 - F::cast_from(0.71414953796510926458e0_f64) * t37634 - F::cast_from(0.57829097596741960692e-3_f64) * t37639 + F::cast_from(0.21951497276451705328e0_f64) * t39495 - F::cast_from(0.97574405393827830187e-2_f64) * t37656 + t41414 - t41415;
    t41416
}
