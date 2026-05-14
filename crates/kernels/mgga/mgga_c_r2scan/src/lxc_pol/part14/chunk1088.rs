//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1088/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1088<F: Float>(t11333: F, t40713: F, t11345: F, t11523: F, t3718: F, t5086: F, t10998: F, t3275: F, t10610: F, t3465: F, t40644: F, t11336: F, t39263: F, t39264: F, t37327: F, t39268: F) -> (F, F, F, F, F, F) {
    let t41788 = 5.0 / 8.0 * t40713 * t11333;
    let t41790 = t11523 * t11345 / 2.0;
    let t41791 = t5086 * t3718;
    let t41794 = 45.0 / 64.0 * t3275 * t41791 * t10998;
    let t41797 = 3.0 * t10610 * t3465 * t40644;
    let t41800 = 3.0 * t39263 * t11336 * t39264;
    let t41803 = 15.0 / 8.0 * t37327 * t11336 * t39268;
    (t41788, t41790, t41794, t41797, t41800, t41803)
}
