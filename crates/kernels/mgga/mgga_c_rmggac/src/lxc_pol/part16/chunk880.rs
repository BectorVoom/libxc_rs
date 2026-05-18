//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 880/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk880<F: Float>(t9219: F, t9223: F, t9225: F, t9229: F, t9236: F, t9675: F, t9678: F, t1970: F, t1971: F, t236: F, t6149: F, t6113: F, t7365: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44493 = F::new(0.5107751987195740728e-4) * t9219;
    let t44494 = F::new(0.212822999466489197e-4) * t9223;
    let t44495 = F::new(0.17961362552795712846e0) * t9225;
    let t44496 = F::new(0.11974241701863808564e0) * t9229;
    let t44498 = F::new(0.1702583995731913576e-4) * t9236;
    let t44499 = F::new(0.4726e1) * t9675;
    let t44500 = F::new(2.0) * t9678;
    let t44580 = t1970 * t1971 * t236 * t6149;
    let t44584 = t7365 * t1971 * t236 * t6113;
    (t44493, t44494, t44495, t44496, t44498, t44499, t44500, t44580, t44584)
}
