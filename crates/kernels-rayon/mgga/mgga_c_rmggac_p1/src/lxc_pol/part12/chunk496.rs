//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 496/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk496(t4187: f64, t1415: f64, t385: f64, t1413: f64, t381: f64, t1425: f64, t1529: f64, t1532: f64, t4155: f64, t4163: f64, t4173: f64, t4182: f64, t4214: f64, t4220: f64, t4336: f64, t4338: f64, t4586: f64, t5385: f64, t5388: f64, t5389: f64, t5392: f64, t5393: f64, t5394: f64, t5395: f64, t5402: f64) -> (f64, f64, f64, f64, f64) {
    let t5403 = 2.0_f64 * t4187;
    let t5404 = t385 * t1415;
    let t5405 = 8.0_f64 * t5404;
    let t5407 = 8.0_f64 * t381 * t1413;
    let t5409 = 8.0_f64 * t385 * t1413;
    let t5410 = t5385 - 0.62182e-1_f64 * t1529 * t1532 - t5388 - 0.93273e-1_f64 * t4182 * t5389 - t4155 - t4163 - t5392 - t5393 - t5394 + 0.186546e0_f64 * t5395 * t4586 + 0.93273e-1_f64 * t1425 * t4173 - t5402 + t5403 + t4336 - t4338 + t4214 - t4220 - t5405 + t5407 - t5409;
    (t5403, t5405, t5407, t5409, t5410)
}
