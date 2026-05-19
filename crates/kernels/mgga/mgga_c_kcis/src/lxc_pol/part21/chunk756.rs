//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 756/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk756<F: Float>(t2308: F, t2311: F, t237: F, t88: F, t2333: F, t2339: F, t2341: F, t661: F, t2371: F, t52: F, t2375: F, t8656: F) -> (F, F, F, F) {
    let t8674 = F::new(0.10685e0) * t237 * t88 * t2308 * t2311;
    let t8678 = F::cast_from(0.48245472966453314466e2_f64) * t2339 * t2333 * t2341 * t661;
    let t8680 = F::new(1.0) / t2371 / t52;
    let t8682 = t8680 * t8656 * t2375;
    (t8674, t8678, t8680, t8682)
}
