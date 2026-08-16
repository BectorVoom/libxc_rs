//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 807/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk807(t1270: f64, t1659: f64, t198: f64, t507: f64, t1625: f64, t541: f64, t3184: f64, t1206: f64, t1268: f64, t2292: f64, t2302: f64, t2310: f64, t3183: f64, t3198: f64, t3209: f64, t3213: f64, t3281: f64, t3307: f64, t3310: f64, t4440: f64, t4441: f64, t4442: f64, t4524: f64, t4525: f64) -> (f64, f64, f64, f64) {
    let t4528 = t1659 * t1270;
    let t4532 = t198 * t507;
    let t4533 = t541 * t1625;
    let t4537 = t3184 * t1625;
    let t4540 = 3.0_f64 * t1206 * t3183 * t4528 + 6.0_f64 * t1206 * t4532 * t4533 - t1268 * t4524 * t4525 + 3.0_f64 * t3183 * t4537 - t2292 + t2302 + t2310 + t3198 - t3209 - t3213 + t3281 + t3307 + t3310 - t4440 - t4441 - t4442;
    (t4528, t4532, t4533, t4540)
}
