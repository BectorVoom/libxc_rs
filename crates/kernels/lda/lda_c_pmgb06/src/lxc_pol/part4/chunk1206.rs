//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1206/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1206<F: Float>(t11944: F, t16077: F, t16083: F, t16087: F, t16090: F, t16092: F, t16094: F, t16095: F, t16099: F, t9408: F, t9410: F, t9412: F, t9417: F, t9418: F, t9422: F, t16100: F, t16101: F, t16102: F, t16103: F, t16105: F, t16107: F, t16109: F, t16112: F, t16114: F, t16117: F, t16121: F, t16122: F, t9424: F, t9426: F, t9429: F) -> (F, F) {
    let t18203 = -0.13298177777777778 * t11944 - t16077 - t16083 - t16087 + t16090 - t16092 - t16094 - t16095 - t16099 - t9408 + t9410 + t9412 - t9417 + 4.0 / 9.0 * t9418 + 4.0 / 3.0 * t9422;
    let t18206 = 4e-21 * t9424 + 16.0 / 81.0 * t9426 + t9429 - t16100 + t16101 - t16102 - t16103 - t16105 - t16107 + t16109 + t16112 + t16114 + t16117 + t16121 + t16122;
    (t18203, t18206)
}
