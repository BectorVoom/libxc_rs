//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 511/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk511<F: Float>(t1265: F, t370: F, t1273: F, t1275: F, t4007: F, t4060: F, t4011: F, t4015: F, t4018: F, t4021: F, t4039: F, t4047: F, t4055: F, t4057: F, t4063: F, t4067: F, t4070: F, t4073: F) -> (F, F, F, F) {
    let t4099 = t1265 * t370;
    let t4100 = F::cast_from(1.0_f64) / t4099;
    let t4101 = t1273 * t1273;
    let t4103 = t4100 * t4101 * t1275;
    let t4108 = F::cast_from(0.40256666666666666667e0_f64) * t4007;
    let t4115 = F::cast_from(0.27595e0_f64) * t4060;
    let t4120 = -F::cast_from(0.1294625e1_f64) * t4039 + F::cast_from(0.258925e1_f64) * t4047 + t4108 + F::cast_from(0.20128333333333333334e0_f64) * t4011 - F::cast_from(0.20128333333333333333e0_f64) * t4015 + F::cast_from(0.60385e0_f64) * t4018 - F::cast_from(0.301925e0_f64) * t4021 + F::cast_from(0.82524375e-1_f64) * t4055 + F::cast_from(0.16504875e0_f64) * t4057 + t4115 + F::cast_from(0.22076e0_f64) * t4063 - F::cast_from(0.5519e-1_f64) * t4067 + F::cast_from(0.33114e0_f64) * t4070 - F::cast_from(0.16557e0_f64) * t4073;
    (t4100, t4101, t4103, t4120)
}
