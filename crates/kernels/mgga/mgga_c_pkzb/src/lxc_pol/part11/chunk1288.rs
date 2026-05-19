//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1288/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1288<F: Float>(t11213: F, t2320: F, t11290: F, t6317: F, t2197: F, t3070: F, t3765: F, t1185: F, t9837: F, t11293: F, t6137: F, t1184: F, t2240: F, t26880: F) -> (F, F, F, F, F, F) {
    let t31357 = t11213 * t2320;
    let t31369 = F::new(6.0) * t6317 * t11290;
    let t31372 = F::new(6.0) * t2197 * t3070 * t3765;
    let t31375 = F::new(6.0) * t2197 * t1185 * t9837;
    let t31377 = F::cast_from(0.48245938496077605201e2_f64) * t6137 * t11293;
    let t31380 = F::cast_from(0.48245938496077605201e2_f64) * t2240 * t26880 * t1184;
    (t31357, t31369, t31372, t31375, t31377, t31380)
}
