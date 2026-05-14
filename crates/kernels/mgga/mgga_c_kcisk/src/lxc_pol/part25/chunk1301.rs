//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1301/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1301<F: Float>(t10487: F, t1791: F, t10798: F, t33031: F, t34022: F, t15874: F, t1799: F, t33017: F, t16600: F, t9679: F, t112176: F, t6680: F, t1871: F, t6944: F, t1895: F, t415: F) -> (F, F, F, F, F, F) {
    let t116645 = t1791 * t10487;
    let t116651 = t33031 * t10798 * t34022;
    let t116656 = t1799 * t33017 * t15874;
    let t116659 = t1799 * t9679 * t16600;
    let t116662 = t1799 * t112176 * t6680;
    let t116664 = t6944 * t1871;
    let t116666 = t415 * t116664 * t1895;
    (t116645, t116651, t116656, t116659, t116662, t116666)
}
