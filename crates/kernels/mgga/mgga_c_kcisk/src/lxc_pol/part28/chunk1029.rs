//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1029/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1029<F: Float>(t645: F, t1755: F, t23052: F, t1751: F, t1758: F, t17697: F, t23304: F, t23716: F, t23726: F, t23733: F, t2442: F, t340: F, t6141: F, t639: F, t642: F, t6707: F, t7186: F, t7196: F, t8773: F, t8781: F, t8787: F) -> (F, F) {
    let t646 = t645 < -0.66725e-1;
    let t23743 = t1755 * t23052;
    let t23748 = piecewise3(t646, 0.0, 10.0 / 9.0 * t340 * t23716 * t642 - 10.0 / 27.0 * t340 * t8773 * t1758 - 20.0 / 27.0 * t340 * t7186 * t2442 + 80.0 / 81.0 * t6141 * t23726 * t6707 + 40.0 / 81.0 * t340 * t1751 * t8781 - 280.0 / 243.0 * t6141 * t17697 * t23733 - 10.0 / 27.0 * t340 * t1751 * t8787 + 40.0 / 81.0 * t6141 * t7196 * t23304 - 10.0 / 27.0 * t340 * t639 * t23743);
    (t23743, t23748)
}
