//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 679/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk679(t124: f64, t3245: f64, t762: f64, t3234: f64, t1218: f64, t521: f64) -> (f64, f64, f64) {
    let t3246 = t124 * t3245;
    let t3247 = t762 * t3246;
    let t3251 = t762 * t124 * t3234;
    let t3255 = 1.0_f64 / t1218 / t521;
    (t3247, t3251, t3255)
}
