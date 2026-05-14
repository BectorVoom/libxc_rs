//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 274/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk274<F: Float>(t1724: F, t1725: F, t1695: F, t1699: F, t45: F, t625: F) -> (F, F, F, F) {
    let t1726 = t1724 * t1725;
    let t1729 = 0.92708333333333333333e-2 * t1695;
    let t1731 = -t1729 - 0.92708333333333333333e-2 * t1699;
    let t1735 = t45 * t625;
    (t1726, t1729, t1731, t1735)
}
