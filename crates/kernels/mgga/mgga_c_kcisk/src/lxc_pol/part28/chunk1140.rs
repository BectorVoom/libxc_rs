//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1140/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1140<F: Float>(t17182: F, t9671: F, t9664: F, t11197: F, t648: F, t1772: F) -> (F, F, F, F) {
    let t32995 = t17182 * t9671;
    let t32996 = t9664 * t32995;
    let t33001 = t11197 * t648;
    let t33002 = t33001 * t1772;
    (t32995, t32996, t33001, t33002)
}
