//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 974/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk974<F: Float>(t25113: F, t77: F, t2251: F, t603: F, t2259: F, t239: F, t2311: F, t76: F, t10298: F, t38: F, t2248: F, t84: F, t2247: F, t607: F, t1927: F, t644: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25114 = t77 * t25113;
    let t25117 = t603 * t2251;
    let t25120 = t603 * t2259;
    let t25137 = 88.0 / 9.0 * t239;
    let t25146 = t76 * t2311;
    let t25150 = t10298 * t38;
    let t25159 = t77 * t84 * t2248;
    let t25162 = t2247 * t607;
    let t25163 = t1927 * t644;
    (t25114, t25117, t25120, t25137, t25146, t25150, t25159, t25162, t25163)
}
