//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1007/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1007(t1265: f64, t5448: f64, t3365: f64, t532: f64, t5380: f64, t1639: f64, t1649: f64, t5407: f64, t1219: f64, t5427: f64, t10193: f64, t1233: f64, t1260: f64, t13059: f64, t13098: f64, t13705: f64, t13763: f64, t13851: f64, t13866: f64, t1640: f64, t220: f64, t3374: f64, t339: f64, t4417: f64, t4460: f64, t4498: f64, t4499: f64, t4508: f64, t4511: f64, t523: f64, t5381: f64, t5408: f64, t5413: f64) -> (f64, f64) {
    let t13888 = t5448 * t1265;
    let t13889 = t3365 * t13888;
    let t13892 = t532 * t5380;
    let t13905 = t1649 * t1639;
    let t13918 = t532 * t5407;
    let t13935 = t1219 * t5427;
    let t13940 = 2.0_f64 * t10193 * t339 * t5381 - t1233 * t13892 * t4508 - 2.0_f64 * t1233 * t13905 * t4508 - t1233 * t13918 * t4508 - t1233 * t13935 * t339 - t1260 * t13851 * t339 - 6.0_f64 * t13059 * t13705 * t13892 - 2.0_f64 * t13098 * t1640 * t339 + 4.0_f64 * t13763 * t4498 * t4499 + t13866 * t220 * t523 + 6.0_f64 * t13892 * t4417 * t4498 + 4.0_f64 * t13905 * t4417 * t4498 + 2.0_f64 * t13918 * t4417 * t4498 - t3374 * t339 * t5408 - t3374 * t339 * t5413 - 2.0_f64 * t339 * t4460 * t4511 - 2.0_f64 * t4460 * t4499 * t4508;
    (t13889, t13940)
}
