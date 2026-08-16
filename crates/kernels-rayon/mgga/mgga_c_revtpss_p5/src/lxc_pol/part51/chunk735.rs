//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 735/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk735(t248: f64, t8486: f64, t8469: f64, t8472: f64, t8473: f64, t8478: f64, t8481: f64) -> f64 {
    let t8487 = t8486 * t248;
    let t8489 = 0.28234466758480466999e-3_f64 * t8469 - 0.8673628188205199462e0_f64 * t8472 * t8473 + 0.57119737665102352616e0_f64 * t8478 * t8481 - 0.1859366460452550541e-3_f64 * t8487;
    t8489
}
