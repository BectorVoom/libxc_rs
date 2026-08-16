//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 984/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk984<F: Float>(t1329: F, t1808: F, t391: F, t4435: F, t1200: F, t1795: F, t11589: F, t3: F, t107: F, t110: F, t11188: F, t11695: F, t1338: F, t1799: F, t199: F, t2804: F, t399: F, t5543: F, t566: F, t84: F, t868: F) -> (F, F) {
    let t11698 = t1329 * t1808;
    let t11700 = t391 * t4435;
    let t11708 = t1795 * t1200;
    let t11710 = t3 * t11589;
    let t11717 = F::cast_from(0.42708890021612717_f64) * t107 * t110 * t11188 - F::cast_from(0.0837628205355044_f64) * t84 * t11695 + F::cast_from(0.5025769232130264_f64) * t11698 + F::cast_from(0.2512884616065132_f64) * t11700 - F::cast_from(0.0837628205355044_f64) * t2804 * t868 - F::cast_from(0.2512884616065132_f64) * t1338 * t1808 - F::cast_from(0.2512884616065132_f64) * t399 * t4435 + F::cast_from(0.2512884616065132_f64) * t11708 - F::cast_from(0.0837628205355044_f64) * t11710 * t199 - F::cast_from(0.2512884616065132_f64) * t5543 * t566 - F::cast_from(0.2512884616065132_f64) * t1799 * t1200;
    (t11710, t11717)
}
