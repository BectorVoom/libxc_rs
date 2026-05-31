//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 862/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk862<F: Float>(t9258: F, t9259: F, t2031: F, t2931: F, t7700: F, t2104: F, t2899: F, t299: F, t5591: F, t5597: F, t5609: F, t5614: F, t5675: F, t7582: F, t7585: F, t7591: F, t7617: F, t7621: F, t7630: F, t7639: F, t7694: F, t9253: F) -> (F, F) {
    let t9260 = t9258 * t9259;
    let t9263 = t2031 * t2931;
    let t9264 = t7700 * t9263;
    let t9267 = -F::cast_from(0.95275595817932748827e-4_f64) * t7582 - t7585 + t7591 + t5591 + t5597 / F::cast_from(162.0_f64) + F::cast_from(0.2540682555144873302e-3_f64) * t5609 + t5614 + t7617 + t7621 / F::cast_from(216.0_f64) - t7630 - t7639 - F::cast_from(0.42874018118069736972e-3_f64) * t299 * t9253 + F::cast_from(0.95275595817932748826e-4_f64) * t5675 + F::cast_from(0.25724410870841842183e-2_f64) * t2104 * t9260 - F::cast_from(0.17149607247227894789e-2_f64) * t2899 * t9264 + t7694;
    (t9263, t9267)
}
