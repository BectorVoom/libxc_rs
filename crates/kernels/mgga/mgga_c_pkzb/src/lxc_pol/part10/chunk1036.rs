//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1036/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1036<F: Float>(t24: F, t1430: F, t2548: F, t507: F, t8734: F, t8739: F, t8742: F, t91: F, t8733: F, t98: F, zeta_threshold: F) -> (F,) {
    let t90 = t24 <= zeta_threshold;
    let t8746 = piecewise3(t90, 0.0, -8.0 / 27.0 * t8734 * t507 - 16.0 / 9.0 * t2548 * t1430 + 4.0 / 9.0 * t8739 * t507 + 4.0 / 3.0 * t91 * t8742);
    let t8748 = (t8733 + t8746) * t98;
    (t8748,)
}
