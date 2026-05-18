//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 481/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk481<F: Float>(t2344: F, t40: F, t1766: F, t1773: F, t1776: F, t1777: F, t1779: F, t2343: F, t85: F, t1036: F, t1045: F, t1049: F, t916: F) -> F {
    let t2345 = t40 * t2344;
    let t2346 = F::new(1.169644679491041) * t1766;
    let t2348 = F::new(0.0003662311007350632) * t1773;
    let t2349 = F::new(2.0) * t1776;
    let t2350 = F::new(8.0) * t1777;
    let t2351 = F::new(8.0) * t1779;
    let t2353 = t2343 * t85;
    let t2354 = F::new(0.019751789702565206) * t2353;
    let t2355 = t2345 + t2354 - t2346 - t2350 - t2351 + t2349 - t2348 + t1036 - t1045 - t1049 - t916;
    t2355
}
