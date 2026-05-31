//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1141/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1141<F: Float>(t136: F, t2247: F, t26178: F, t32593: F, t10309: F, t32596: F, t32586: F, t239: F, t8435: F, t8623: F, t10301: F, t8619: F) -> (F, F, F, F, F, F, F) {
    let t121629 = t2247 * t26178 * t136;
    let t121630 = t121629 * t32593;
    let t121632 = t10309 * t32596;
    let t121633 = t121632 * t32586;
    let t121638 = F::cast_from(55.0_f64) / F::cast_from(81.0_f64) * t2247 * t8435 * t239 * t8623;
    let t121644 = t10301 * t32596 * t8623;
    let t121646 = t8619 * t136;
    (t121629, t121630, t121632, t121633, t121638, t121644, t121646)
}
