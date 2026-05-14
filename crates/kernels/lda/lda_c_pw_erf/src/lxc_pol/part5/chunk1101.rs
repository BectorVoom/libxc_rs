//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1101/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1101<F: Float>(t504: F, t7520: F, t348: F, t519: F, t9351: F, t1318: F, t1319: F, t549: F, t7404: F, t18474: F, t2171: F, t6419: F, t6423: F, t1325: F, t1326: F, t494: F, t7655: F) -> (F, F, F, F, F, F) {
    let t22881 = t7520 * t504;
    let t22885 = 8.0 / 15.0 * t519 * t9351 * t22881 * t348;
    let t22889 = 8.0 / 45.0 * t1318 * t1319 * t7404 * t549;
    let t22890 = 16.0 / 45.0 * t18474;
    let t22892 = 32.0 / 27.0 * t2171 * t6419;
    let t22894 = 16.0 / 9.0 * t2171 * t6423;
    let t22898 = 8.0 / 45.0 * t1325 * t1326 * t7655 * t494;
    (t22885, t22889, t22890, t22892, t22894, t22898)
}
