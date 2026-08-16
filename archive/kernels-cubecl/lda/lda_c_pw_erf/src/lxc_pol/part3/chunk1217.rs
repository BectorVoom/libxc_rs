//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1217/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1217<F: Float>(t4647: F, t544: F, t1524: F, t2123: F, t1394: F, t1982: F, t1518: F, t2066: F, t211: F, t4703: F, t595: F, t14344: F, t14347: F, t14350: F, t14352: F, t14353: F, t14354: F, t14355: F, t14357: F) -> (F, F, F, F, F, F) {
    let t14359 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t4647 * t544;
    let t14360 = t1524 * t2123;
    let t14361 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t14360;
    let t14363 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1982 * t1394;
    let t14365 = t211 * t1518 * t2066;
    let t14366 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t14365;
    let t14368 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t4703 * t595;
    let t14369 = -t14344 - t14347 + t14350 - t14352 - t14353 + t14354 + t14355 - t14357 - t14359 - t14361 + t14363 + t14366 - t14368;
    (t14359, t14361, t14363, t14366, t14368, t14369)
}
