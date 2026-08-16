//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 325/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk325<F: Float>(t1011: F, t1036: F, t1017: F, t1028: F, t1033: F, t1040: F) -> (F, F, F) {
    let t1056 = F::cast_from(0.516475e0_f64) * t1011;
    let t1059 = F::cast_from(0.104195e0_f64) * t1036;
    let t1061 = F::cast_from(0.3529725e1_f64) * t1028 - t1056 + F::cast_from(0.516475e0_f64) * t1017 + F::cast_from(0.6311625e0_f64) * t1033 - t1059 + F::cast_from(0.104195e0_f64) * t1040;
    (t1056, t1059, t1061)
}
