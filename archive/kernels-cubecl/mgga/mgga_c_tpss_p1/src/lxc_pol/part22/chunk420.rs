//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 420/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk420<F: Float>(t1441: F, t318: F, t1409: F, t1416: F, t1419: F, t1422: F, t898: F, t901: F) -> (F, F) {
    let t1442 = t1441 * t318;
    let t1448 = F::cast_from(0.258925e1_f64) * t1416 - t898 - F::cast_from(0.301925e0_f64) * t1409 + F::cast_from(0.16504875e0_f64) * t1419 - t901 - F::cast_from(0.82785e-1_f64) * t1422;
    (t1442, t1448)
}
