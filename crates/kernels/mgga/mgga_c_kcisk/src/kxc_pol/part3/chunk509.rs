//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 509/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk509<F: Float>(t1311: F, t24: F, t1248: F, t3575: F, t1249: F, t3579: F, t3583: F, t4011: F, t4015: F, t4018: F, t4021: F, t4039: F, t4047: F, t4049: F, t4055: F, t4057: F, t4061: F, t4063: F) -> (F, F, F, F, F) {
    let t4065 = t24 * t1311;
    let t4067 = t1248 * t4065 * t3575;
    let t4070 = t1248 * t1249 * t3579;
    let t4073 = t1248 * t1249 * t3583;
    let t4075 = -F::new(0.9494625e0) * t4039 + F::new(0.1898925e1) * t4047 + t4049 + F::cast_from(0.19931111111111111111e0_f64) * t4011 - F::cast_from(0.19931111111111111111e0_f64) * t4015 + F::cast_from(0.59793333333333333334e0_f64) * t4018 - F::cast_from(0.29896666666666666667e0_f64) * t4021 + F::new(0.15358125e0) * t4055 + F::new(0.3071625e0) * t4057 + t4061 + F::cast_from(0.21908444444444444444e0_f64) * t4063 - F::cast_from(0.5477111111111111111e-1_f64) * t4067 + F::cast_from(0.32862666666666666666e0_f64) * t4070 - F::cast_from(0.16431333333333333333e0_f64) * t4073;
    (t4065, t4067, t4070, t4073, t4075)
}
