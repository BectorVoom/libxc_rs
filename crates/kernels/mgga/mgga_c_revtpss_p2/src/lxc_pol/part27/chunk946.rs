//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 946/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk946<F: Float>(t3022: F, t3034: F, t3006: F, t3011: F, t4733: F, t981: F, t2935: F, t945: F, t2967: F, t941: F, t2966: F, t307: F) -> (F, F, F, F, F) {
    let t11394 = F::cast_from(0.51947577317044391276e2_f64) * t3022 * t3034;
    let t11396 = t3011 * t3006 * t4733;
    let t11398 = F::cast_from(0.51947577317044391277e2_f64) * t981 * t11396;
    let t11399 = t2935 * t945;
    let t11404 = t941 * t2967;
    let t11408 = F::cast_from(1.0_f64) / t2966 / t307;
    (t11394, t11398, t11399, t11404, t11408)
}
