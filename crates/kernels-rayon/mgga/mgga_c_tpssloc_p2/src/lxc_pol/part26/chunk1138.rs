//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1138/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1138(t23204: f64, t6555: f64, t23164: f64, t6572: f64, t6562: f64, t6624: f64, t798: f64, t1911: f64, t2719: f64, t10110: f64, t2742: f64, t6571: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23205 = t23204 * t6555;
    let t23206 = t23164 * t23205;
    let t23207 = 0.16449340668482264365e-1_f64 * t23206;
    let t23208 = t23204 * t6572;
    let t23209 = t6562 * t23208;
    let t23211 = t798 * t6624;
    let t23214 = t1911 * t2719;
    let t23215 = t10110 * t23214;
    let t23218 = t6571 * t2742;
    (t23205, t23207, t23208, t23209, t23211, t23215, t23218)
}
