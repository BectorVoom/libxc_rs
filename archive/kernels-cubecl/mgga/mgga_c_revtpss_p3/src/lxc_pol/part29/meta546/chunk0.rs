//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1883/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1883<F: Float>(t25878: F, t96239: F, t26230: F, t9670: F, t25895: F, t94633: F, t25899: F, t94639: F, t1358: F, t2439: F, t7506: F, t785: F) -> (F, F, F, F, F, F, F, F) {
    let t96240 = t25878 * t96239;
    let t96242 = t26230 * t9670;
    let t96243 = t25895 * t96242;
    let t96245 = t26230 * t94633;
    let t96246 = t25899 * t96245;
    let t96248 = t26230 * t94639;
    let t96249 = t25899 * t96248;
    let t96253 = t2439 * t785 * t7506 * t1358;
    (t96240, t96242, t96243, t96245, t96246, t96248, t96249, t96253)
}
