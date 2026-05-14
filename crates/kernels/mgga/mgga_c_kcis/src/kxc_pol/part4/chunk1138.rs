//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1138/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1138<F: Float>(t1889: F, t3761: F, t3809: F, t1319: F, t5477: F, t10269: F, t1102: F, t16379: F, t16384: F, t16390: F, t16393: F, t16398: F, t16401: F, t16404: F, t16408: F, t16410: F, t16413: F, t16418: F, t16423: F, t16427: F, t16432: F, t16436: F, t16438: F, t16441: F, t16443: F, t4587: F) -> (F,) {
    let t16447 = t3761 * t1889 * t3809;
    let t16451 = t3761 * t5477 * t1319;
    let t16454 = -0.65704296666666666667e-3 * t1102 * t16379 - 0.10950716111111111111e-2 * t1102 * t16384 + 0.59133867e-2 * t1102 * t16390 + 0.39422577999999999999e-2 * t1102 * t16393 + 0.13140859333333333333e-2 * t1102 * t16398 + 0.21901432222222222222e-3 * t16401 - 0.59133867e-2 * t1102 * t16404 - t16408 + t16410 - 0.36958666875e-3 * t1102 * t16413 + 0.295669335e-2 * t1102 * t16418 - 0.295669335e-2 * t1102 * t16423 - 0.1478346675e-2 * t1102 * t16427 - 0.19711289e-2 * t1102 * t16432 + t16436 - 0.13140859333333333333e-2 * t10269 * t16438 - 0.32852148333333333333e-3 * t16441 - 0.43802864444444444444e-2 * t4587 * t16443 + 0.98556445e-3 * t1102 * t16447 - 0.39422578e-2 * t4587 * t16451;
    (t16454,)
}
