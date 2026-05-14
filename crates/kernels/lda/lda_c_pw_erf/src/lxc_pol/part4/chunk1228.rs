//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1228/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1228<F: Float>(t3794: F, t6344: F, t10603: F, t10607: F, t10620: F, t3968: F, t6711: F, t12403: F, t4488: F, t12322: F, t13459: F, t3965: F, t6710: F, t12516: F, t4494: F, t4501: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18213 = 16.0 / 27.0 * t3794 * t6344;
    let t18214 = 8.0 / 405.0 * t10603;
    let t18215 = 8.0 / 243.0 * t10607;
    let t18216 = 16.0 / 405.0 * t10620;
    let t18217 = t6711 * t3968;
    let t18220 = 64.0 / 45.0 * t4488 * t12403 * t18217;
    let t18223 = 32.0 / 27.0 * t4488 * t12322 * t18217;
    let t18227 = 64.0 / 45.0 * t3965 * t6710 * t6711 * t13459;
    let t18228 = t6711 * t12516;
    let t18231 = 64.0 / 45.0 * t3965 * t4494 * t18228;
    let t18234 = 32.0 / 27.0 * t3965 * t4501 * t18228;
    (t18213, t18214, t18215, t18216, t18220, t18223, t18227, t18231, t18234)
}
