//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 634/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk634(t28: f64, t265: f64, t504: f64, t2756: f64, t3643: f64, t1081: f64, t1260: f64, t2250: f64, t3231: f64, t506: f64, t52: f64, t607: f64, t873: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t3644 = piecewise3(t505, t3643, t2756);
    let t3651 = piecewise3(t401, t2756 * t28 / 2.0_f64 + t873 * t1081 + t265 * t3231 / 2.0_f64, t3644 * t52 / 2.0_f64 - t1260 * t607 - t506 * t2250 / 2.0_f64);
    (t3644, t3651)
}
