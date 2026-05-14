//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 301/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk301<F: Float>(t1341: F, t1440: F, t1415: F, t1411: F, t1299: F, t470: F, t468: F, t415: F, t382: F, t394: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t1441 = t1341 * t1440;
    let t1442 = t1415 * t1441;
    let t1443 = t1411 * t1442;
    let t1445 = sigma0 * t1299;
    let t1446 = t1445 * t470;
    let t1447 = t468 * t1446;
    let t1448 = t415 * t1447;
    let t1450 = t394 * t382;
    (t1441, t1442, t1443, t1445, t1446, t1447, t1448, t1450)
}
