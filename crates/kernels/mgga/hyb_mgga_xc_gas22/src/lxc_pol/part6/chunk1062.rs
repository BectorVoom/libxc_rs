//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1062/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1062<F: Float>(t7: F, t10097: F, t10136: F, t10151: F, t9861: F, t214: F, t4086: F, t675: F, t1289: F, t1318: F, t191: F, t3984: F, t2024: F, t3926: F, t6479: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t10154 = piecewise3::<f64>(t9, F::new(0.0), t9861 + t10097 + t10136 + t10151);
    let t10158 = t4086 * t214;
    let t10159 = t10158 * t675;
    let t10163 = t1289 * t1318;
    let t10164 = t10163 * t675;
    let t10168 = t191 * t3984;
    let t10169 = t10168 * t675;
    let t10174 = t2024 * t6479 * t3926;
    (t10154, t10158, t10159, t10163, t10164, t10168, t10169, t10174)
}
