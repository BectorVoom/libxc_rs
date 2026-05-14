//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1258/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1258<F: Float>(t1302: F, t6597: F, t1397: F, t6601: F, t14313: F, t14316: F, t1381: F, t1466: F, t571: F, t6968: F, t14339: F, t14343: F, t14346: F, t14349: F, t14351: F, t544: F, t6631: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t18694 = 4.0 / 15.0 * t6597 * t1302;
    let t18695 = t6601 * t1397;
    let t18696 = 16.0 / 45.0 * t18695;
    let t18697 = 32.0 / 135.0 * t14313;
    let t18698 = 16.0 / 45.0 * t14316;
    let t18702 = 4.0 / 5.0 * t571 * t1466 * t6968 * t1381;
    let t18703 = 32.0 / 45.0 * t14339;
    let t18704 = 32.0 / 45.0 * t14343;
    let t18705 = 16.0 / 45.0 * t14346;
    let t18706 = 16.0 / 45.0 * t14349;
    let t18707 = 16.0 / 135.0 * t14351;
    let t18709 = 4.0 / 15.0 * t6631 * t544;
    (t18694, t18696, t18697, t18698, t18702, t18703, t18704, t18705, t18706, t18707, t18709)
}
