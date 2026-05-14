//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 422/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk422<F: Float>(t2789: F, t705: F, t415: F, t2782: F, t2785: F, t752: F, t733: F, t736: F, t1800: F, t748: F) -> (F, F, F, F, F, F, F) {
    let t2790 = t705 * t2789;
    let t2791 = t415 * t2790;
    let t2793 = -0.10416666666666666667e-1 * t2782 * t2785 + 0.24872916666666666666e-2 * t2791;
    let t2794 = t2793 * t752;
    let t2795 = t733 * t736;
    let t2797 = t1800 * t748;
    let t2799 = t2795 / 16.0 - t2797 / 128.0;
    (t2790, t2791, t2793, t2794, t2795, t2797, t2799)
}
