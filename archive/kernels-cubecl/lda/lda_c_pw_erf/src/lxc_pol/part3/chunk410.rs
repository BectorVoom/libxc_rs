//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 410/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk410<F: Float>(t1529: F, t211: F, t1471: F, t1474: F, t1478: F, t1482: F, t1490: F, t1500: F, t1510: F, t1515: F, t1517: F, t1521: F, t1526: F, t1528: F) -> (F, F) {
    let t1531 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t211 * t1529;
    let t1532 = -t1471 + t1474 + t1478 + t1482 + t1490 + t1500 + t1510 + t1515 + t1517 - t1521 + t1526 + t1528 - t1531;
    (t1531, t1532)
}
