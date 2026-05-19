//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 191/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk191<F: Float>(t50: F, t277: F, t391: F, t419: F, t421: F, t475: F, t490: F, t498: F, t95: F, t7: F, t8: F, zeta_threshold: F) -> (F, F, F) {
    let t51 = t50 <= zeta_threshold;
    let t501 = -t391 + t419 + t421 + F::cast_from(0.25844881434903430496e-2_f64) * t95 * t277 * t475 + t490 * t498 / F::new(2.0);
    let t502 = piecewise3::<F>(t51, zeta_threshold, t50);
    let t507 = t8 * t7;
    let t508 = F::new(1.0) / t507;
    (t501, t502, t508)
}
