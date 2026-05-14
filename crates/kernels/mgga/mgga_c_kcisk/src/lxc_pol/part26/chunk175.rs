//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 175/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk175<F: Float>(t240: F, t505: F, t551: F, t555: F, t157: F, t32: F, t5: F, t28: F, t14: F, t15: F) -> (F, F, F, F, F, F) {
    let t559 = t505 + t240 * (t551 * t555 - t505);
    let t812 = 0.11073577833333333333e-2 * t5 * t157 * t32;
    let t813 = t28 * t28;
    let t814 = 1.0 / t813;
    let t815 = t14 * t814;
    let t816 = 1.0 / t15;
    (t559, t812, t813, t814, t815, t816)
}
