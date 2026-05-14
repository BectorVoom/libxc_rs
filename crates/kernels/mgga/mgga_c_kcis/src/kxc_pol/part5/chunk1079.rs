//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1079/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1079<F: Float>(t20141: F, t376: F, t1170: F, t18458: F, t381: F, t1189: F, t1175: F, t6696: F, t1796: F, t284: F, t5078: F, t19945: F, t19948: F, t19951: F, t19954: F, t19958: F, t19961: F, t19963: F, t19965: F, t19967: F, t19970: F, t20131: F, t20134: F, t20137: F, t20139: F) -> (F, F, F, F, F) {
    let t20142 = t376 * t20141;
    let t20143 = t1170 * t20142;
    let t20145 = t18458 * t381;
    let t20146 = t20145 * t1189;
    let t20148 = t1175 * t6696;
    let t20149 = t1170 * t20148;
    let t20151 = t1796 * t284;
    let t20152 = t20151 * t5078;
    let t20154 = t19945 / 96.0 + t19948 / 864.0 + t19951 / 12.0 + t19954 / 8.0 + t19958 / 24.0 - t19961 / 64.0 + t19963 / 128.0 - t19965 / 72.0 - t19967 / 96.0 - t19970 / 9.0 + t20131 / 16.0 - t20134 / 3.0 + t20137 / 4.0 - 2.0 / 9.0 * t20139 - t20143 / 16.0 + t20146 / 256.0 + t20149 / 6.0 - t20152 / 36.0;
    (t20143, t20146, t20149, t20152, t20154)
}
