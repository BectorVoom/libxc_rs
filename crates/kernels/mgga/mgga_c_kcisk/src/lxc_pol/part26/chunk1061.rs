//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1061/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1061<F: Float>(t28074: F, t28094: F, t28114: F, t28134: F, t15087: F, t15094: F, t1611: F, t1620: F, t21345: F, t22056: F, t2347: F, t28034: F, t28036: F, t28046: F, t28049: F, t28053: F, t4530: F, t4535: F, t555: F, t6604: F, t6607: F, t6638: F, t8436: F, t8455: F) -> (F, F) {
    let t28136 = t28074 + t28094 + t28114 + t28134;
    let t28138 = 2.0 * t15087 * t8436 - 6.0 * t15094 * t28046 - t1611 * t28136 - t1620 * t28036 + 4.0 * t21345 * t6607 - 2.0 * t22056 * t2347 + t28034 * t555 + 4.0 * t28049 * t4535 + 2.0 * t28053 * t4535 - t4530 * t8455 - 2.0 * t6604 * t6638;
    (t28136, t28138)
}
