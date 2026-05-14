//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 656/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk656<F: Float>(t1611: F, t1620: F, t2347: F, t240: F, t4530: F, t4535: F, t555: F, t6240: F, t6242: F, t6243: F, t6246: F, t6395: F, t6602: F, t6604: F, t6607: F, t6638: F) -> (F,) {
    let t6642 = t6240 - t6242 - t6243 + t6246 - t6395 + t240 * (-t1611 * t6638 - t1620 * t6604 - t2347 * t4530 + 2.0 * t4535 * t6607 + t555 * t6602 - t6240 + t6242 + t6243 - t6246 + t6395);
    (t6642,)
}
