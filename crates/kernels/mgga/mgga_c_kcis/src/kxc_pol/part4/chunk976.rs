//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 976/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk976<F: Float>(t1072: F, t2630: F, t2844: F, t89: F, t740: F, t113: F, t9494: F, t1068: F, t829: F, t2635: F, t331: F, t1071: F, t160: F) -> (F, F, F, F, F, F, F, F) {
    let t10091 = t1072 * t2630;
    let t10093 = t89 * t2844;
    let t10096 = t740 * t2844;
    let t10097 = t10096 * t2630;
    let t10099 = t113 * t9494;
    let t10102 = t1068 * t829;
    let t10104 = t331 * t2635;
    let t10108 = t160 * t1071;
    (t10091, t10093, t10096, t10097, t10099, t10102, t10104, t10108)
}
