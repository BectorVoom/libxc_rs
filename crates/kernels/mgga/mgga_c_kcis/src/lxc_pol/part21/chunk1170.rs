//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1170/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1170<F: Float>(t1748: F, t26832: F, t303: F, t27856: F, t7687: F, t27974: F, t7696: F, t10995: F, t2175: F, t26685: F, t27958: F, t4981: F, t93211: F, t93216: F, t93366: F, t95976: F, t95980: F, t95985: F, t96428: F, t96430: F) -> (F, F) {
    let t96433 = t303 * t1748 * t26832;
    let t96449 = 0.46336805555555555556e-3 * t7687 * t27856;
    let t96451 = 0.12356481481481481482e-2 * t7696 * t27974;
    let t96452 = t96428 - 0.13265555555555555555e-1 * t96430 + 0.24320185185185185185e-1 * t96433 + 0.61836467013888888888e-4 * t93366 * t27958 + 0.61836467013888888888e-4 * t26685 * t95976 + 0.30918233506944444444e-4 * t26685 * t95980 + 0.41224311342592592592e-4 * t26685 * t95985 - 0.67960648148148148147e-2 * t4981 * t10995 * t2175 + 0.16581944444444444444e-2 * t93211 - 0.88437037037037037034e-2 * t93216 + t96449 - t96451;
    (t96433, t96452)
}
