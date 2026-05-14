//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 971/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk971<F: Float>(t11994: F, t33696: F, t33258: F, t3781: F, t11356: F, t9563: F, t9934: F, t474: F, t8837: F, t10031: F, t3402: F, t1084: F, t9923: F, t10043: F, t11945: F, t11387: F, t3363: F) -> (F, F, F, F, F, F, F, F) {
    let t33741 = t33696 * t11994;
    let t33743 = t33258 * t3781;
    let t33746 = t9563 * t11356 * t9934;
    let t33748 = t474 * t8837;
    let t33750 = t3402 * t33748 * t10031;
    let t33753 = t1084 * t33748 * t9923;
    let t33755 = t10043 * t11945;
    let t33757 = t3363 * t11387;
    (t33741, t33743, t33746, t33748, t33750, t33753, t33755, t33757)
}
