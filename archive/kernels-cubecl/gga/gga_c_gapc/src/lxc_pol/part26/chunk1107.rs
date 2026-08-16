//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1107/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1107<F: Float>(t11994: F, t33696: F, t33258: F, t3781: F, t11356: F, t9563: F, t9934: F, t474: F, t8837: F, t10031: F, t3402: F, t1084: F, t9923: F) -> (F, F, F, F, F, F) {
    let t33741 = t33696 * t11994;
    let t33743 = t33258 * t3781;
    let t33746 = t9563 * t11356 * t9934;
    let t33748 = t474 * t8837;
    let t33750 = t3402 * t33748 * t10031;
    let t33753 = t1084 * t33748 * t9923;
    (t33741, t33743, t33746, t33748, t33750, t33753)
}
