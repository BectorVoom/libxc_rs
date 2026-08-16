//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 939/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk939(t31805: f64, t32237: f64, t32240: f64, t1419: f64, t8477: f64, t1385: f64, t9656: f64, t1444: f64, t8578: f64, t3999: f64, t4075: f64, t1398: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32244 = t31805 * t32237;
    let t32246 = 0.25389723392137995738e-1_f64 * t32244 * t32240;
    let t32247 = t8477 * t1419;
    let t32250 = t9656 * t1385;
    let t32252 = t32250 * t8578 * t1444;
    let t32255 = t4075 * t3999;
    let t32257 = t8578 * t1398 * t543;
    (t32244, t32246, t32247, t32250, t32252, t32255, t32257)
}
