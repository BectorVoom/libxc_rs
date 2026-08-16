//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1849/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1849<F: Float>(t3712: F, t372: F, t3630: F, t12705: F, t5341: F, t3720: F, t5333: F, t1263: F, t675: F) -> (F, F, F, F, F, F) {
    let t12867 = t372 * t3712;
    let t12868 = t12867 * t3630;
    let t12871 = t12705 * t5341;
    let t12872 = t3720 * t12871;
    let t12875 = t12705 * t5333;
    let t12876 = t3720 * t12875;
    let t12879 = t675 * t1263;
    (t12868, t12871, t12872, t12875, t12876, t12879)
}
