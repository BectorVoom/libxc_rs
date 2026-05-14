//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 822/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk822<F: Float>(t3209: F, t8483: F, t2508: F, t7226: F, t13179: F, t7129: F, t10667: F, t795: F, t948: F, t13209: F, t7137: F, t1841: F, t8878: F, t9748: F, t10053: F, t1024: F) -> (F, F, F, F, F, F, F, F) {
    let t43240 = t8483 * t3209;
    let t43243 = 0.46143157380853345701e-1 * t2508 * t7226 * t43240;
    let t43244 = t7129 * t13179;
    let t43246 = t795 * t10667;
    let t43248 = t2508 * t43246 * t948;
    let t43254 = 0.10254034973522965712e-1 * t7137 * t13209;
    let t43257 = 0.25635087433807414279e-2 * t1841 * t8878 * t9748;
    let t43260 = 0.76905262301422242837e-2 * t2508 * t10053 * t1024;
    (t43240, t43243, t43244, t43246, t43248, t43254, t43257, t43260)
}
