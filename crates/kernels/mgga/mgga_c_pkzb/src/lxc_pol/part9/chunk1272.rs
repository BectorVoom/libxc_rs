//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1272/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1272<F: Float>(t22225: F, t22308: F, t881: F, t890: F, t898: F, t18427: F, t18430: F, t18433: F, t18445: F, t18448: F, t18554: F, t18555: F, t22190: F, t22193: F, t22196: F, t22199: F, t22202: F, t22205: F, t22207: F, t22209: F, t22215: F, t22217: F, t22220: F, t22222: F) -> (F, F, F) {
    let t22309 = t22225 + t22308;
    let t22313 = F::new(0.5848223622634646207e0) * t898 * t881 * t22309 * t890;
    let t22331 = F::new(0.427258125e1) * t22190 - F::new(0.230371875e0) * t22193 - F::new(0.3560484375e1) * t22196 + F::new(0.1151859375e0) * t22199 - F::new(0.28483875e1) * t22202 + F::new(0.46074375e0) * t22205 - F::new(0.28483875e1) * t22207 - F::new(0.9494625e0) * t22209 + t18554 - F::new(0.27903555555555555556e1) * t18427 + F::new(0.11958666666666666667e1) * t18430 - F::new(0.29896666666666666667e0) * t18433 + t18555 + F::new(0.82156666666666666666e0) * t18448 + F::new(0.46074375e0) * t22215 + F::new(0.15358125e0) * t22217 + F::new(0.427258125e1) * t22220 - F::new(0.230371875e0) * t22222 - F::new(0.21908444444444444445e1) * t18445;
    (t22309, t22313, t22331)
}
