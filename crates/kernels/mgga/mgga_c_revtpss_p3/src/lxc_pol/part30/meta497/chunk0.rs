//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1851/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1851<F: Float>(t5: F, t30: F, t265: F, t393: F, t26798: F, t117: F, t2126: F, t2327: F, t25743: F, t2129: F, t2258: F, t25459: F, t45: F, t606: F, t7594: F, t2138: F, t3650: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t26799 = piecewise3::<F>(t8, F::new(0.0), t26798);
    let t26800 = t26799 * t117;
    let t26804 = t2126 * t2327;
    let t26809 = piecewise3::<F>(t394, F::new(0.0), t25743);
    let t26816 = piecewise3::<F>(t120, t25459, t26809 * t45 / F::new(2.0) + t7594 * t606 + t2129 * t2258 / F::new(2.0));
    let t26817 = t3650 * t2138;
    (t26799, t26800, t26804, t26809, t26816, t26817)
}
