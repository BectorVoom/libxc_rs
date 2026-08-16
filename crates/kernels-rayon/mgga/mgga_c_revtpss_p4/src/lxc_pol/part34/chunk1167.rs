//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1167/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1167(t30: f64, t1469: f64, t1996: f64, t29726: f64, t29931: f64, t45: f64, t5825: f64, t7856: f64, t33: f64, t5966: f64, t1963: f64, t25759: f64, t29598: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t29938 = piecewise3(t120, t29726, t29931 * t45 / 2.0_f64 + t7856 * t1469 + t1996 * t5825 / 2.0_f64);
    let t29939 = t33 * t5966;
    let t29940 = t1963 * t29939;
    let t29946 = t25759 * t29598;
    (t29938, t29939, t29940, t29946)
}
