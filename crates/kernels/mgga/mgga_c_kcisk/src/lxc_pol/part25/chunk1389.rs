//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1389/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1389<F: Float>(t109152: F, t109154: F, t109160: F, t109162: F, t109165: F, t110815: F, t110817: F, t117263: F, t118563: F, t2360: F, t2670: F, t2776: F, t32876: F, t33319: F, t4573: F, t564: F, t567: F, t9904: F) -> (F,) {
    let t118576 = t109152 - t109154 - t564 * t567 * (t117263 + t118563) / 16.0 - t564 * t2360 * t33319 / 16.0 + t9904 * t32876 / 16.0 - t109160 + t109162 - t2776 * t4573 * t2670 / 16.0 - t109165 + t110815 - t110817;
    (t118576,)
}
