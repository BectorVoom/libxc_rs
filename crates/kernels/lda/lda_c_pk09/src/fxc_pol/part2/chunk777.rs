//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 777/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk777<F: Float>(t119: F, t2379: F, t10: F, t2378: F, t88: F, t975: F, t2340: F, t747: F, t106: F, t2393: F, t1011: F, t1052: F, t2210: F, t2214: F, t2394: F, t4098: F, t4346: F, t4348: F, t709: F, t713: F, t7706: F, t7768: F, t7776: F, t7962: F, t98: F) -> (F, F) {
    let t9183 = t2379 * t119;
    let t9187 = t2378 * t88 * t10;
    let t9188 = t975 * t9187;
    let t9204 = t747 * t2340;
    let t9205 = t106 * t9204;
    let t9209 = t2393 * t119;
    let t9217 = t9183 * t709 / 6.0 - t9188 * t98 / 6.0 + t4098 * t2210 / 6.0 + t1052 * t7962 / 6.0 + t1052 * t7768 / 6.0 + t4098 * t2214 / 6.0 + t1052 * t7776 / 6.0 + t1052 * t7706 / 6.0 + t4346 / 9.0 + t9205 / 9.0 + t9183 * t713 / 6.0 + t9209 * t713 / 6.0 + t9209 * t709 / 6.0 - t4348 / 9.0 - t2394 * t1011 / 6.0;
    (t9204, t9217)
}
