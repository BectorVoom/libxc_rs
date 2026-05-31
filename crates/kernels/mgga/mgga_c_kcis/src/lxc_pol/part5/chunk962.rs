//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 962/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk962<F: Float>(t2877: F, t984: F, t2810: F, t296: F, t1080: F, t2475: F, t2844: F, t89: F, t740: F, t113: F, t9494: F, t1068: F, t829: F) -> (F, F, F, F, F, F, F) {
    let t9970 = t984 * t2877;
    let t9985 = F::cast_from(1.0_f64) / t2810 / t296;
    let t10033 = t2475 * t1080;
    let t10093 = t89 * t2844;
    let t10096 = t740 * t2844;
    let t10099 = t113 * t9494;
    let t10102 = t1068 * t829;
    (t9970, t9985, t10033, t10093, t10096, t10099, t10102)
}
