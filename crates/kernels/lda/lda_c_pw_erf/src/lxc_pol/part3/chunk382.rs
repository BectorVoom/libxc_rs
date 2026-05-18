//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 382/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk382<F: Float>(t1391: F, t1392: F, t186: F, t185: F, t514: F, t550: F) -> (F, F, F, F) {
    let t1393 = t1391 * t1392;
    let t1394 = t186 * t1393;
    let t1396 = F::new(4.0) / F::new(15.0) * t185 * t1394;
    let t1397 = t514 * t550;
    (t1393, t1394, t1396, t1397)
}
