//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 797/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk797(t1613: f64, t72: f64, t732: f64, t177: f64, t737: f64, t3200: f64, t3301: f64, t2292: f64, t2302: f64, t2310: f64, t3198: f64, t3209: f64, t3213: f64, t3281: f64, t3307: f64, t3310: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4435 = t1613 * t72;
    let t4436 = t4435 * t732;
    let t4437 = 0.18311447306006545054e-3_f64 * t4436;
    let t4438 = t1613 * t177;
    let t4439 = t4438 * t737;
    let t4440 = 0.5848223622634646207e0_f64 * t4439;
    let t4441 = 4.0_f64 * t3200;
    let t4442 = 4.0_f64 * t3301;
    let t4443 = -t4437 - t4440 + t3198 - t4441 + t2310 - t3209 - t3213 - t4442 + t3307 + t3281 + t3310 - t2292 + t2302;
    (t4435, t4437, t4438, t4440, t4441, t4442, t4443)
}
