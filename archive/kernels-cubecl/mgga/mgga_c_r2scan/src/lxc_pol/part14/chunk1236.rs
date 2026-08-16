//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1236/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1236<F: Float>(t11333: F, t40713: F, t11345: F, t11523: F, t3718: F, t5086: F, t10998: F, t3275: F, t10610: F, t3465: F, t40644: F, t11336: F, t39263: F, t39264: F) -> (F, F, F, F, F) {
    let t41788 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t40713 * t11333;
    let t41790 = t11523 * t11345 / F::cast_from(2.0_f64);
    let t41791 = t5086 * t3718;
    let t41794 = F::cast_from(45.0_f64) / F::cast_from(64.0_f64) * t3275 * t41791 * t10998;
    let t41797 = F::cast_from(3.0_f64) * t10610 * t3465 * t40644;
    let t41800 = F::cast_from(3.0_f64) * t39263 * t11336 * t39264;
    (t41788, t41790, t41794, t41797, t41800)
}
