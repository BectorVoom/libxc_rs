//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 358/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk358<F: Float>(t1118: F, t1143: F, t1124: F, t1135: F, t1140: F, t1147: F) -> (F, F, F) {
    let t1182 = F::cast_from(0.301925e0_f64) * t1118;
    let t1185 = F::cast_from(0.82785e-1_f64) * t1143;
    let t1187 = F::cast_from(0.258925e1_f64) * t1135 - t1182 + F::cast_from(0.301925e0_f64) * t1124 + F::cast_from(0.16504875e0_f64) * t1140 - t1185 + F::cast_from(0.82785e-1_f64) * t1147;
    (t1182, t1185, t1187)
}
