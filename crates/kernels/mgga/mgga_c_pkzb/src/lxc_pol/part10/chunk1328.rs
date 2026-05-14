//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1328/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1328<F: Float>(t25959: F, t25964: F, t25967: F, t25971: F, t26027: F, t26282: F, t26285: F, t26356: F, t26364: F, t26366: F, t26369: F, t26371: F, t26374: F, t25620: F, t25669: F, t25819: F, t25850: F, t25881: F, t25914: F, t25957: F) -> (F,) {
    let t26375 = -t25959 - t25964 - t25967 - t25971 - t26027 + t26364 - t26282 + t26285 - t26366 + t26369 + t26371 - t26374 + t26356;
    let t26378 = t25620 + t25669 + t25819 + t25850 + t25881 + t25914 + t25957 + t26375;
    (t26378,)
}
