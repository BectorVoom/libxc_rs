//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 86/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk86<F: Float>(t367: F, t372: F, t374: F, t365: F, t6: F, t8: F, t103: F, t61: F) -> (F, F, F, F) {
    let t375 = t367 * t372 * t374;
    let t377 = F::new(0.58482233974552040708e0) * t365 * t375;
    let t378 = t6 * t8;
    let t380 = t61 * t378 * t103;
    (t375, t377, t378, t380)
}
