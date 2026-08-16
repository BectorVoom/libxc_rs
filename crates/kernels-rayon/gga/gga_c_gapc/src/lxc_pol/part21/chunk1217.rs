//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1217/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1217(t11356: f64, t26007: f64, t9304: f64, t2993: f64, t3001: f64, t33158: f64, t35031: f64, t35034: f64, t35036: f64, t35039: f64, t35041: f64, t35045: f64, t35048: f64, t35051: f64, t35056: f64) -> f64 {
    let t35059 = t9304 * t11356 * t26007;
    let t35062 = t2993 * t33158 * t3001;
    let t35064 = -0.20241536458333333334e-4_f64 * t35031 - 0.2209926229259557733e-7_f64 * t35034 - 0.25340269868817520618e-3_f64 * t35036 - 0.12650960286458333334e-5_f64 * t35039 - 0.28985453471303521737e-5_f64 * t35041 - 0.19336854506021130164e-8_f64 * t35045 - 0.40483072916666666668e-4_f64 * t35048 - 0.49240895655712845849e-7_f64 * t35051 + 0.78584976712469872988e-8_f64 * t35056 + 0.21103240995305505364e-7_f64 * t35059 - 0.49522272202316919254e-5_f64 * t35062;
    t35064
}
