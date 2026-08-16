//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1101/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1101<F: Float>(t2039: F, t671: F, t1401: F, t3938: F, t3941: F, t577: F, t7056: F, t7222: F, t7230: F, t1184: F, t460: F, t33: F, t3953: F) -> (F, F, F, F) {
    let t7235 = t2039 * t671;
    let t7240 = F::cast_from(0.45e1_f64) * t7222 * t577 + F::cast_from(0.135e2_f64) * t7230 * t671 + F::cast_from(0.135e2_f64) * t3938 * t2039 + F::cast_from(27.0_f64) * t3941 * t7235 + F::cast_from(0.135e2_f64) * t1401 * t7056;
    let t7319 = t1184 * t460;
    let t7428 = t3953 * t33;
    (t7235, t7240, t7319, t7428)
}
