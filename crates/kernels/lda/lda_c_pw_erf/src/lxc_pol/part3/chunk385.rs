//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 385/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk385<F: Float>(t1402: F, t1403: F, t186: F, t211: F, t653: F, t656: F, t156: F, t254: F) -> (F, F, F, F, F) {
    let t1404 = t1402 * t1403;
    let t1405 = t186 * t1404;
    let t1407 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t211 * t1405;
    let t1409 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t653 * t656;
    let t1410 = t254 * t156;
    (t1404, t1405, t1407, t1409, t1410)
}
