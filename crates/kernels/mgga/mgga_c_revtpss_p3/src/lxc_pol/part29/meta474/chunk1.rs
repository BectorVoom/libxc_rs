//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1745/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1745<F: Float>(t33: F, t265: F, t502: F, t26625: F, t2085: F, t2258: F, t26665: F, t57: F, t606: F, t7468: F, t26633: F, t2051: F, t2327: F, t2107: F, t25177: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t26666 = piecewise3::<F>(t503, F::new(0.0), t26625);
    let t26673 = piecewise3::<F>(t400, t26665, t26666 * t57 / F::new(2.0) - t7468 * t606 - t2085 * t2258 / F::new(2.0));
    let t26674 = t26633 + t26673;
    let t26676 = t2051 * t2327;
    let t26679 = t2107 * t25177;
    (t26666, t26674, t26676, t26679)
}
