//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 256/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk256<F: Float>(t1179: F, t439: F, t1118: F, t1143: F, t1124: F, t1135: F, t1140: F, t1147: F) -> (F, F, F, F) {
    let t1180 = t439 * t1179;
    let t1182 = F::new(0.301925e0) * t1118;
    let t1185 = F::new(0.82785e-1) * t1143;
    let t1187 = F::new(0.258925e1) * t1135 - t1182 + F::new(0.301925e0) * t1124 + F::new(0.16504875e0) * t1140 - t1185 + F::new(0.82785e-1) * t1147;
    (t1180, t1182, t1185, t1187)
}
