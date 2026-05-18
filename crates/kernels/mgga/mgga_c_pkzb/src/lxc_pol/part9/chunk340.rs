//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 340/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk340<F: Float>(t1174: F, t834: F, t841: F, t1167: F, t334: F, t218: F, t219: F, t1169: F, t839: F, t846: F) -> (F, F, F, F, F) {
    let t1175 = t834 * t1174;
    let t1178 = t841 * t1174;
    let t1180 = t334 * t1167;
    let t1182 = t218 * t219 * t1180;
    let t1184 = F::new(0.1898925e1) * t1175 - t839 + F::new(0.8969e0) * t1169 + F::new(0.3071625e0) * t1178 - t846 + F::new(0.24647e0) * t1182;
    (t1175, t1178, t1180, t1182, t1184)
}
