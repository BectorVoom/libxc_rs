//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 898/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk898<F: Float>(t1340: F, t13407: F, t1411: F, t13367: F, t13372: F, t13375: F, t13380: F, t13385: F, t13387: F, t13389: F, t13392: F, t13397: F, t13400: F, t13404: F) -> (F, F) {
    let t13408 = t1340 * t13407;
    let t13409 = t1411 * t13408;
    let t13411 = -F::cast_from(0.2653111111111111111e-1_f64) * t13367 + F::cast_from(0.16581944444444444444e-2_f64) * t13372 + F::cast_from(0.49745833333333333332e-2_f64) * t13375 - F::cast_from(0.66327777777777777776e-2_f64) * t13380 - F::cast_from(0.74618749999999999998e-2_f64) * t13385 + F::cast_from(0.99491666666666666664e-2_f64) * t13387 + F::cast_from(0.2653111111111111111e-1_f64) * t13389 - F::cast_from(0.16581944444444444444e-2_f64) * t13392 - F::cast_from(0.16581944444444444444e-2_f64) * t13397 + t13400 - F::cast_from(0.72960555555555555553e-1_f64) * t13404 + F::cast_from(0.48640370370370370369e-1_f64) * t13409;
    (t13409, t13411)
}
