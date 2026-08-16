//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1961/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1961<F: Float>(t15968: F, t182: F, t1787: F, t2516: F, t17: F, t12097: F, t12100: F, t12111: F, t12120: F, t184: F, t2663: F, t5157: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15970 = F::cast_from(0.19751673498613801407e-1_f64) * t15968 * t182;
    let t15971 = t1787 * t2516;
    let t15972 = t17 * t15971;
    let t15973 = F::cast_from(0.4883052614935078681e-3_f64) * t12097;
    let t15974 = F::cast_from(0.18311447306006545054e-3_f64) * t12100;
    let t15975 = F::cast_from(0.21687162600603479684e-1_f64) * t12111;
    let t15976 = F::cast_from(4.0_f64) * t12120;
    let t15977 = t15968 * t184;
    let t15978 = t17 * t15977;
    let t15979 = t5157 * t2663;
    (t15970, t15971, t15972, t15973, t15974, t15975, t15976, t15977, t15978, t15979)
}
