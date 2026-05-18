//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1258/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1258<F: Float>(t122278: F, t27873: F, t122274: F, t125663: F, t121029: F, t121044: F, t122351: F, t122355: F, t122443: F, t125659: F, t32674: F, t32678: F, t34231: F, t7926: F) -> F {
    let t128665 = t122278 * t27873;
    let t128673 = t122274 * t27873;
    let t128676 = F::new(0.150583822711895824e-3) * t125663;
    let t128677 = -t122351 - F::new(0.28912093960683998207e-1) * t128665 + F::new(0.57119737665102352616e0) * t34231 * t32674 + F::new(0.57119737665102352616e0) * t34231 * t32678 + t121029 + F::new(0.8673628188205199462e0) * t122443 * t7926 + F::new(0.51405703062096148813e-1) * t128673 - F::new(0.225875734067843736e-2) * t125659 + t128676 - t121044 - t122355;
    t128677
}
