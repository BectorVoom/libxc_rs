//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1843/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1843<F: Float>(t30: F, t1996: F, t2258: F, t25459: F, t25744: F, t45: F, t606: F, t7194: F, t2394: F, t33: F, t2411: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t25751 = piecewise3::<F>(t120, t25459, t25744 * t45 / F::new(2.0) + t7194 * t606 + t1996 * t2258 / F::new(2.0));
    let t25752 = t33 * t2394;
    let t25759 = t2411 * t33;
    (t25751, t25752, t25759)
}
