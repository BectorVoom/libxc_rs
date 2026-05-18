//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 897/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk897<F: Float>(t13394: F, t3776: F, t1340: F, t1411: F, t10500: F, t472: F, t3913: F, t470: F, t1440: F, t1415: F, t382: F, t1286: F) -> (F, F, F, F, F, F) {
    let t13395 = t3776 * t13394;
    let t13396 = t1340 * t13395;
    let t13397 = t1411 * t13396;
    let t13399 = t10500 * t472;
    let t13400 = F::new(0.73697530864197530862e-3) * t13399;
    let t13401 = t3913 * t470;
    let t13402 = t13401 * t1440;
    let t13403 = t1415 * t13402;
    let t13404 = t1411 * t13403;
    let t13406 = t3913 * t382;
    let t13407 = t13406 * t1286;
    (t13397, t13399, t13400, t13401, t13404, t13407)
}
