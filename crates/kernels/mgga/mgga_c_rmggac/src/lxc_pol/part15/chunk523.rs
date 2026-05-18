//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 523/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk523<F: Float>(t352: F, t6557: F, t321: F, t6522: F, t333: F, t305: F, t326: F, t3814: F, t3839: F, t4669: F, t5148: F, t5162: F, t5259: F, t5266: F, t5271: F, t5942: F, t5945: F, t5954: F, t5957: F, t5963: F, t6308: F, t6311: F, t6315: F, t6332: F, t6335: F, t6339: F, t6382: F, t6387: F, t6482: F, t6523: F, t6530: F, t797: F, t838: F) -> (F, F, F, F) {
    let t6558 = t6557 * t352;
    let t6561 = t6522 * t321;
    let t6564 = t6522 * t333;
    let t6567 = -F::new(0.23948483403727617128e0) * t5148 * t6523 + F::new(0.71845450211182851384e0) * t5271 * t6382 - F::new(0.14369090042236570277e1) * t5162 * t6387 - F::new(0.35922725105591425692e0) * t4669 * t6530 - F::new(0.11974241701863808564e0) * t326 * t5945 - F::new(0.59871208509319042821e-1) * t326 * t6339 - F::new(0.11974241701863808564e0) * t305 * t5963 - F::new(0.11974241701863808564e0) * t326 * t6308 - F::new(0.71845450211182851384e0) * t3814 * t5954 + F::new(0.11974241701863808564e1) * t3839 * t5957 + F::new(0.17961362552795712846e0) * t797 * t6332 - F::new(0.23948483403727617128e0) * t838 * t6335 + F::new(0.11974241701863808564e0) * t305 * t6482 + F::new(0.35922725105591425692e0) * t797 * t6311 + F::new(0.35922725105591425692e0) * t797 * t6315 + F::new(0.11974241701863808564e0) * t305 * t5942 + F::new(0.23948483403727617128e0) * t5266 * t6558 + F::new(0.23948483403727617128e0) * t5259 * t6561 - F::new(0.35922725105591425692e0) * t4669 * t6564;
    (t6558, t6561, t6564, t6567)
}
