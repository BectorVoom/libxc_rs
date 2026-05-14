//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1159/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1159<F: Float>(t18427: F, t18468: F, t22230: F, t22302: F, t27262: F, t27295: F, t31067: F, t31088: F, t834: F, t841: F, t218: F, t219: F, t3026: F, t3730: F, t1167: F, t9795: F) -> (F, F, F, F) {
    let t31239 = t18468 - 28.0 / 27.0 * t18427 - 28.0 / 9.0 * t22230 + t22302 + 4.0 / 3.0 * t27295 - t27262 - t31067 / 3.0 + t31088;
    let t31240 = t834 * t31239;
    let t31242 = t841 * t31239;
    let t31250 = t218 * t219 * t3026 * t3730;
    let t31254 = t218 * t219 * t1167 * t9795;
    (t31240, t31242, t31250, t31254)
}
