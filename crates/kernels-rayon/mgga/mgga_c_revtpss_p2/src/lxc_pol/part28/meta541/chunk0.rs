//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1990/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1990(t2452: f64, t9720: f64, t675: f64, t886: f64, t11006: f64, t256: f64, t10115: f64, t251: f64, t2410: f64, t3335: f64, t11198: f64, t340: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40688 = t9720 * t2452;
    let t41040 = t675 * t886;
    let t41077 = 1.0_f64 / t11006 / t256;
    let t41117 = t10115 * t251;
    let t41153 = t2410 * t2410;
    let t41154 = 1.0_f64 / t41153;
    let t41936 = t3335 * t3335;
    let t41937 = 1.0_f64 / t41936;
    let t42058 = 1.0_f64 / t11198 / t340;
    (t40688, t41040, t41077, t41117, t41154, t41937, t42058)
}
