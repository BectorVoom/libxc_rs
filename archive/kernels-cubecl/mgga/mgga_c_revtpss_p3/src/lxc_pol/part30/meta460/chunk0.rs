//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1755/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1755<F: Float>(t2315: F, t84: F, t77: F, t2251: F, t603: F, t2259: F, t239: F, t2311: F, t76: F, t10298: F, t38: F, t2248: F) -> (F, F, F, F, F, F, F) {
    let t25113 = t84 * t2315;
    let t25114 = t77 * t25113;
    let t25117 = t603 * t2251;
    let t25120 = t603 * t2259;
    let t25137 = F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t239;
    let t25146 = t76 * t2311;
    let t25150 = t10298 * t38;
    let t25159 = t77 * t84 * t2248;
    (t25114, t25117, t25120, t25137, t25146, t25150, t25159)
}
