//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1345/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1345<F: Float>(t2707: F, t28178: F, t33986: F, t109152: F, t109154: F, t109160: F, t109162: F, t109165: F, t1629: F, t2053: F, t2356: F, t25289: F, t2670: F, t2776: F, t34673: F, t34676: F, t34679: F, t35532: F, t564: F, t6650: F, t806: F, t8471: F, t9904: F) -> (F, F, F) {
    let t120907 = t28178 * t2707;
    let t120929 = 4.0 * t33986;
    let t120957 = t109152 - t109154 + t2356 * t34676 / 8.0 - t2776 * t6650 * t2670 / 8.0 - t564 * t1629 * t35532 / 16.0 - t2776 * t8471 * t2053 / 16.0 - t109160 + t2356 * t34673 / 8.0 - t2776 * t25289 * t806 / 16.0 + t109162 - t109165 + t9904 * t34679 / 8.0;
    (t120907, t120929, t120957)
}
