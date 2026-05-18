//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1216/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1216<F: Float>(t1894: F, t7489: F, t1898: F, t2743: F, t1902: F, t2746: F, t5797: F, t713: F, t7510: F, t694: F, t7518: F, t5771: F, t7312: F) -> (F, F, F, F, F, F) {
    let t21220 = F::new(3.0) * t7489 * t1894;
    let t21221 = t2743 * t1898;
    let t21223 = F::new(0.48245938496077605201e2) * t21221 * t1902;
    let t21225 = F::new(1.0) * t2746 * t5797;
    let t21226 = t7510 * t713;
    let t21229 = t7518 * t694;
    let t21233 = F::new(18.0) * t5771 * t7312;
    (t21220, t21223, t21225, t21226, t21229, t21233)
}
