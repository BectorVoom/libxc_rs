//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1427/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1427<F: Float>(t26917: F, t18979: F, t18984: F, t18990: F, t18995: F, t22616: F, t22619: F, t23739: F, t23741: F, t23742: F, t246: F, t26805: F, t22608: F, t7877: F, t22648: F, t22650: F, t897: F) -> (F, F, F) {
    let t26918 = 0.4051561992e0 * t26917;
    let t26919 = t18979 - 0.285764e-1 * t246 * t26805 - t18984 + t23739 + t18990 + 0.5143752e0 * t22616 - 0.1714584e0 * t22619 - t23741 + t23742 - t18995 + t26918;
    let t26921 = t7877 * t22608;
    let t26922 = 0.4051561992e0 * t26921;
    let t26924 = t22648 * t897 * t22650;
    (t26919, t26922, t26924)
}
