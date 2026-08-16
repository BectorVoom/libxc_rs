//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1249/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1249(t11270: f64, t11273: f64, t25530: f64, t35379: f64, t35384: f64, t35386: f64, t35388: f64, t35390: f64, t35393: f64, t35395: f64, t35397: f64, t35400: f64, t35404: f64, t35406: f64, t35409: f64) -> f64 {
    let t35412 = t11270 * t25530 * t11273;
    let t35414 = 0.3475929712541504153e-3_f64 * t35379 - 0.12441355264518896277e-6_f64 * t35384 - 0.43449121406768801912e-4_f64 * t35386 - 0.86898242813537603824e-4_f64 * t35388 - 0.86898242813537603825e-3_f64 * t35390 - 0.22776267492663374277e-4_f64 * t35393 - 0.3475929712541504153e-3_f64 * t35395 + 0.2697466287336896452e-3_f64 * t35397 - 0.3475929712541504153e-3_f64 * t35400 - 0.86898242813537603824e-4_f64 * t35404 + 0.70121379086208999512e-5_f64 * t35406 - 0.12653481940368541265e-5_f64 * t35409 - 0.7381197798548315738e-6_f64 * t35412;
    t35414
}
