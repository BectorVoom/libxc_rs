//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 667/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk667<F: Float>(t3815: F, t4035: F, t1409: F, t3841: F, t1075: F, t317: F, t522: F, t3106: F, t323: F, t526: F, t3110: F, t534: F, t333: F, t4016: F, t532: F, t833: F) -> (F, F, F, F, F, F, F, F) {
    let t4036 = t4035 * t3815;
    let t4039 = t1409 * t3841;
    let t4047 = 0.8197e-2 * t317 * t1075 * t522;
    let t4050 = 0.21133333333333333333e-2 * t323 * t3106 * t526;
    let t4051 = t3110 * t534;
    let t4053 = 0.16804375e-4 * t333 * t4051;
    let t4054 = 0.23911438650126355246e-1 * t4016;
    let t4055 = t532 * t833;
    (t4036, t4039, t4047, t4050, t4051, t4053, t4054, t4055)
}
