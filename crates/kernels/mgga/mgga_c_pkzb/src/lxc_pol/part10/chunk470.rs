//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 470/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk470<F: Float>(t12: F, t1643: F, t1837: F, t1646: F, t652: F, zeta_threshold: F) -> (F,) {
    let t84 = t12 <= zeta_threshold;
    let t1838 = t1837 * t1643;
    let t1843 = piecewise3(t84, 0.0, 4.0 / 9.0 * t1838 - t652 * t1646 / 3.0);
    (t1843,)
}
