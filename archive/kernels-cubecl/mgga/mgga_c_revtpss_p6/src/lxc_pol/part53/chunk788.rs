//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 788/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk788<F: Float>(t248: F, t8486: F, t8469: F, t8472: F, t8473: F, t8478: F, t8481: F) -> F {
    let t8487 = t8486 * t248;
    let t8489 = F::cast_from(0.28234466758480466999e-3_f64) * t8469 - F::cast_from(0.8673628188205199462e0_f64) * t8472 * t8473 + F::cast_from(0.57119737665102352616e0_f64) * t8478 * t8481 - F::cast_from(0.1859366460452550541e-3_f64) * t8487;
    t8489
}
