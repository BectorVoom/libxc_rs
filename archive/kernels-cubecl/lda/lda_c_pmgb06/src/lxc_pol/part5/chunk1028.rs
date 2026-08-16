//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1028/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1028<F: Float>(t1972: F, t6131: F, t6268: F, t6536: F, t1981: F, t1982: F, t6130: F, t18020: F, t835: F, t1977: F, t6134: F, t19278: F, t19280: F, t19282: F, t19284: F, t19286: F, t19289: F, t19291: F) -> (F, F, F, F, F, F) {
    let t19293 = t1972 * t6131 / F::cast_from(15.0_f64);
    let t19295 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t6268 * t6536;
    let t19298 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1981 * t6130 * t1982;
    let t19300 = t18020 * t835 / F::cast_from(15.0_f64);
    let t19302 = t6134 * t1977 / F::cast_from(15.0_f64);
    let t19303 = t19278 + t19280 + t19282 - t19284 - t19286 - t19289 + t19291 + t19293 - t19295 - t19298 + t19300 + t19302;
    (t19293, t19295, t19298, t19300, t19302, t19303)
}
