//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1221/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1221<F: Float>(t30: F, t265: F, t393: F, t128014: F, t128060: F, t127592: F, t127912: F, t127939: F, t127976: F, t1469: F, t32535: F, t34127: F, t4186: F, t45: F, t606: F, t8671: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t128061 = t128014 + t128060;
    let t128062 = piecewise3::<F>(t394, F::new(0.0), t128061);
    let t128069 = piecewise3::<F>(t120, t127592 + t127912 + t127939 + t127976, t128062 * t45 / F::new(2.0) + t32535 * t1469 / F::new(2.0) + t34127 * t606 / F::new(2.0) + t8671 * t4186 / F::new(2.0));
    (t128061, t128069)
}
