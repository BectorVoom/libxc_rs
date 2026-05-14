//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1039/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1039<F: Float>(t12: F, t24: F, t5158: F, t1064: F, t1430: F, t207: F, t3510: F, t3512: F, t439: F, t8729: F, t1165: F, t333: F, t3725: F, t3727: F, t507: F, t8742: F, zeta_threshold: F) -> (F, F) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t8795 = 0.17315859105681463759e2 * t5158;
    let t8805 = piecewise3(t84, 0.0, 8.0 / 27.0 * t3510 * t439 - 8.0 / 9.0 * t1064 * t1430 - 2.0 / 9.0 * t3512 * t439 + 2.0 / 3.0 * t207 * t8729);
    let t8815 = piecewise3(t90, 0.0, 8.0 / 27.0 * t3725 * t507 + 8.0 / 9.0 * t1165 * t1430 - 2.0 / 9.0 * t3727 * t507 + 2.0 / 3.0 * t333 * t8742);
    let t8817 = t8805 / 2.0 + t8815 / 2.0;
    (t8795, t8817)
}
