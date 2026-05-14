//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 589/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk589<F: Float>(t1060: F, t2488: F, t2487: F, t5101: F, t1824: F, t1648: F, t2372: F, t2365: F, t821: F) -> (F, F, F, F, F) {
    let t6743 = t2488 * t1060;
    let t6746 = t5101 * t2487;
    let t6747 = t6746 * t1824;
    let t6750 = t2372 * t1648;
    let t6756 = t821 * t2365;
    (t6743, t6746, t6747, t6750, t6756)
}
