//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1294/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1294<F: Float>(t56817: F, t79: F, t33960: F, t9515: F, t32401: F, t33767: F, t109514: F, t33770: F, t32439: F, t109494: F, t33905: F, t9536: F, t115002: F, t33781: F, t3936: F, t2331: F, t442: F) -> (F, F, F, F, F, F, F, F, F) {
    let t115592 = t56817 * t79;
    let t115596 = t9515 * t33960;
    let t115606 = 0.13402777777777777778e-2 * t33767 * t32401;
    let t115645 = t109514 * t33770;
    let t115646 = t32439 * t115645;
    let t115661 = 0.11574074074074074074e-2 * t9536 * t109494 * t33905;
    let t115663 = 0.11574074074074074074e-2 * t9536 * t115002;
    let t115666 = t3936 * t33781;
    let t115667 = t2331 * t442;
    (t115592, t115596, t115606, t115645, t115646, t115661, t115663, t115666, t115667)
}
