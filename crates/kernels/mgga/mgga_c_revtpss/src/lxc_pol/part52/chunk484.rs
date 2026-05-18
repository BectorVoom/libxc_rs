//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 484/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk484<F: Float>(t1263: F, t159: F, t635: F, t2304: F, t1126: F, t1130: F, t1129: F, t418: F, t408: F, t406: F, t409: F, t3356: F) -> (F, F, F, F, F, F, F) {
    let t3360 = t159 * t1263;
    let t3361 = t635 * t635;
    let t3362 = F::new(1.0) / t3361;
    let t3367 = F::new(1.0) / t2304;
    let t3379 = t1126 * t1130;
    let t3382 = t1129 * t418;
    let t3383 = F::new(1.0) / t3382;
    let t3384 = t408 * t3383;
    let t3390 = F::new(1.0) / t409 / t406;
    let t3394 = F::new(4.0) / F::new(9.0) * t3356;
    (t3360, t3362, t3367, t3379, t3384, t3390, t3394)
}
