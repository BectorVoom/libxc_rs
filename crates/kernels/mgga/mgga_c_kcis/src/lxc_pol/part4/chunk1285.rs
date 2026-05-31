//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1285/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1285<F: Float>(t16065: F, t5425: F, t1889: F, t3761: F, t3809: F, t1319: F, t5477: F, t10269: F, t1102: F, t16379: F, t16384: F, t16390: F, t16393: F, t16398: F, t16401: F, t16404: F, t16408: F, t16410: F, t16413: F, t16418: F, t16423: F, t16427: F, t16432: F, t16436: F, t16438: F, t16441: F, t4587: F) -> F {
    let t16443 = t5425 * t16065;
    let t16447 = t3761 * t1889 * t3809;
    let t16451 = t3761 * t5477 * t1319;
    let t16454 = -F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t16379 - F::cast_from(0.10950716111111111111e-2_f64) * t1102 * t16384 + F::cast_from(0.59133867e-2_f64) * t1102 * t16390 + F::cast_from(0.39422577999999999999e-2_f64) * t1102 * t16393 + F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t16398 + F::cast_from(0.21901432222222222222e-3_f64) * t16401 - F::cast_from(0.59133867e-2_f64) * t1102 * t16404 - t16408 + t16410 - F::cast_from(0.36958666875e-3_f64) * t1102 * t16413 + F::cast_from(0.295669335e-2_f64) * t1102 * t16418 - F::cast_from(0.295669335e-2_f64) * t1102 * t16423 - F::cast_from(0.1478346675e-2_f64) * t1102 * t16427 - F::cast_from(0.19711289e-2_f64) * t1102 * t16432 + t16436 - F::cast_from(0.13140859333333333333e-2_f64) * t10269 * t16438 - F::cast_from(0.32852148333333333333e-3_f64) * t16441 - F::cast_from(0.43802864444444444444e-2_f64) * t4587 * t16443 + F::cast_from(0.98556445e-3_f64) * t1102 * t16447 - F::cast_from(0.39422578e-2_f64) * t4587 * t16451;
    t16454
}
