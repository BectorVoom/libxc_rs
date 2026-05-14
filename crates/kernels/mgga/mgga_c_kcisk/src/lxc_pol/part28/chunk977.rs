//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 977/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk977<F: Float>(t22729: F, t26: F, t6771: F, t6817: F, t4716: F, t8522: F, t1648: F, t10663: F, t8504: F, t22603: F, t22605: F, t22608: F, t22718: F, t22721: F, t22724: F, t22727: F) -> (F, F, F, F, F) {
    let t22730 = t26 * t22729;
    let t22734 = t6817 * t6771;
    let t22736 = t4716 * t8522;
    let t22737 = t22736 * t1648;
    let t22740 = t10663 * t8504;
    let t22741 = t22740 * t1648;
    let t22743 = 0.11038e0 * t22718 - 0.49671e0 * t22721 - 0.66228e0 * t22724 + 0.16557e0 * t22727 - 0.27595e-1 * t22730 - 0.258925e1 * t22605 - 0.1294625e1 * t22608 + 0.16504875e0 * t22734 + 0.82524375e-1 * t22737 + 0.19419375e1 * t22603 - 0.412621875e-1 * t22741;
    (t22730, t22734, t22737, t22741, t22743)
}
