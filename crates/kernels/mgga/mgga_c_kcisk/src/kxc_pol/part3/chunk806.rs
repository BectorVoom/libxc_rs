//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 806/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk806<F: Float>(t13394: F, t3776: F, t1340: F, t1411: F, t10500: F, t472: F, t3913: F, t470: F, t1440: F, t1415: F, t382: F, t1286: F, t13367: F, t13372: F, t13375: F, t13380: F, t13385: F, t13387: F, t13389: F, t13392: F) -> (F, F, F, F, F, F) {
    let t13395 = t3776 * t13394;
    let t13396 = t1340 * t13395;
    let t13397 = t1411 * t13396;
    let t13399 = t10500 * t472;
    let t13400 = 0.73697530864197530862e-3 * t13399;
    let t13401 = t3913 * t470;
    let t13402 = t13401 * t1440;
    let t13403 = t1415 * t13402;
    let t13404 = t1411 * t13403;
    let t13406 = t3913 * t382;
    let t13407 = t13406 * t1286;
    let t13408 = t1340 * t13407;
    let t13409 = t1411 * t13408;
    let t13411 = -0.2653111111111111111e-1 * t13367 + 0.16581944444444444444e-2 * t13372 + 0.49745833333333333332e-2 * t13375 - 0.66327777777777777776e-2 * t13380 - 0.74618749999999999998e-2 * t13385 + 0.99491666666666666664e-2 * t13387 + 0.2653111111111111111e-1 * t13389 - 0.16581944444444444444e-2 * t13392 - 0.16581944444444444444e-2 * t13397 + t13400 - 0.72960555555555555553e-1 * t13404 + 0.48640370370370370369e-1 * t13409;
    (t13397, t13399, t13401, t13404, t13409, t13411)
}
