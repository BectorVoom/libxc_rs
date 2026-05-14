//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 817/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk817<F: Float>(t2740: F, t4398: F, t4401: F, t4406: F, t4408: F, t4412: F, t2325: F, t2953: F, t2329: F, t939: F, t2849: F, t462: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5974 = 0.5848223397455204 * t2740;
    let t5975 = 0.021687161765563047 * t4398;
    let t5976 = 24.0 * t4401;
    let t5977 = 2.0 * t4406;
    let t5978 = 40.0 * t4408;
    let t5979 = 2.339289358982082 * t4412;
    let t5982 = t2953 * t2325;
    let t5987 = t939 * t2329;
    let t5992 = -2.0 * t462 - 6.0 * t2849;
    (t5974, t5975, t5976, t5977, t5978, t5979, t5982, t5987, t5992)
}
