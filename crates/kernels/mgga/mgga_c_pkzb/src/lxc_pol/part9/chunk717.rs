//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 717/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk717<F: Float>(t12: F, t1646: F, t5093: F, t5094: F, t5097: F, t5100: F, t87: F, t139: F, t24: F, t1652: F, t507: F, zeta_threshold: F) -> (F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t5104 = piecewise3(t84, 0.0, -8.0 / 27.0 * t5093 * t5094 + 4.0 / 3.0 * t5097 * t1646 + 4.0 / 3.0 * t87 * t5100);
    let t5106 = 1.0 / t139 / t24;
    let t5107 = t1652 * t507;
    (t5104, t5106, t5107)
}
