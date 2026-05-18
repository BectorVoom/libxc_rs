//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1277/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1277<F: Float>(t18474: F, t2171: F, t6419: F, t6423: F, t1325: F, t1326: F, t494: F, t7655: F, t6348: F, t4738: F, t6323: F, t6327: F) -> (F, F, F, F, F, F, F) {
    let t22890 = F::new(16.0) / F::new(45.0) * t18474;
    let t22892 = F::new(32.0) / F::new(27.0) * t2171 * t6419;
    let t22894 = F::new(16.0) / F::new(9.0) * t2171 * t6423;
    let t22898 = F::new(8.0) / F::new(45.0) * t1325 * t1326 * t7655 * t494;
    let t22900 = F::new(4.0) / F::new(9.0) * t2171 * t6348;
    let t22902 = F::new(8.0) / F::new(15.0) * t4738 * t6323;
    let t22904 = F::new(4.0) / F::new(15.0) * t2171 * t6327;
    (t22890, t22892, t22894, t22898, t22900, t22902, t22904)
}
