//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 820/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk820<F: Float>(t2990: F, t1775: F, t344: F, t1799: F, t339: F, t3002: F, t3004: F, t1: F, t1798: F, t397: F, t3010: F, t3018: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5694 = F::cast_from(34.631511798751724_f64) * t2990;
    let t5695 = t344 * t1775;
    let t5696 = F::cast_from(8.0_f64) * t5695;
    let t5697 = t339 * t1799;
    let t5698 = F::cast_from(8.0_f64) * t5697;
    let t5699 = F::cast_from(0.0001831155503675316_f64) * t3002;
    let t5700 = F::cast_from(0.0004883081343134176_f64) * t3004;
    let t5701 = t1798 * t1;
    let t5702 = t5701 * t397;
    let t5703 = F::cast_from(0.0003662311007350632_f64) * t5702;
    let t5704 = F::cast_from(4.0_f64) * t3010;
    let t5705 = F::cast_from(2.0_f64) * t3018;
    (t5694, t5695, t5696, t5697, t5698, t5699, t5700, t5701, t5702, t5703, t5704, t5705)
}
