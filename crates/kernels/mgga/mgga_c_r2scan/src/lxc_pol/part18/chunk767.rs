//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 767/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk767<F: Float>(t1509: F, t898: F, t41: F, t1531: F, t2463: F, t2: F, t2483: F, t464: F, t2333: F, t2850: F, t2271: F, t2810: F) -> (F, F, F, F, F) {
    let t7030 = t898 * t1509;
    let t7031 = t41 * t7030;
    let t7032 = t2463 * t1531;
    let t7034 = t2483 * t2;
    let t7035 = t7034 * t464;
    let t7036 = F::new(0.36622894612013090108e-3) * t7035;
    let t7040 = t2850 * t2333;
    let t7048 = F::new(0.4726e1) * t2271 * t2810;
    (t7031, t7032, t7036, t7040, t7048)
}
