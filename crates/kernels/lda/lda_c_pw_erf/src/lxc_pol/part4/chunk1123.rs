//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1123/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1123<F: Float>(t11: F, t1349: F, t16447: F, t16452: F, t3633: F, t16456: F, t10102: F, t16461: F, t13708: F, t13710: F, t13712: F, t13714: F, t13722: F, t13724: F, t13726: F, t13731: F, t13736: F) -> (F, F, F, F, F) {
    let t16486 = t11 * t1349 * t16447;
    let t16489 = t11 * t3633 * t16452;
    let t16492 = t11 * t3633 * t16456;
    let t16495 = t11 * t10102 * t16461;
    let t16506 = 0.14396666666666666 * t16486 + 0.47988888888888886 * t16489 - 0.03999074074074074 * t16492 - 0.10664197530864197 * t16495 - 0.047988888888888886 * t13708 + 0.12797037037037037 * t13710 + 0.14396666666666666 * t13712 - 0.04265679012345679 * t13714 - 0.047988888888888886 * t13722 - 0.09597777777777777 * t13724 + 0.19195555555555555 * t13726 + 0.07464938271604939 * t13731 + 1.135737037037037 * t13736;
    (t16486, t16489, t16492, t16495, t16506)
}
