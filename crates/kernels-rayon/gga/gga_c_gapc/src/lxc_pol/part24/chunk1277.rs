//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1277/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1277(t1617: f64, t3873: f64, t4915: f64, t10529: f64, t10541: f64, t15430: f64, t3859: f64, t35379: f64, t35384: f64, t35386: f64, t35388: f64, t35390: f64, t35393: f64, t35395: f64, t35397: f64, t35400: f64, t35404: f64, t35406: f64, t35409: f64, t35412: f64) -> (f64, f64, f64, f64) {
    let t37352 = 6.0_f64 * t4915 * t3873 * t1617;
    let t37354 = 8.0_f64 * t10529 * t10541;
    let t37356 = 2.0_f64 * t15430 * t3859;
    let t37370 = 0.13903718850166016612e-2_f64 * t35379 - 0.49765421058075585109e-6_f64 * t35384 - 0.17379648562707520765e-3_f64 * t35386 - 0.3475929712541504153e-3_f64 * t35388 - 0.3475929712541504153e-2_f64 * t35390 - 0.9110506997065349711e-4_f64 * t35393 - 0.13903718850166016612e-2_f64 * t35395 + 0.10789865149347585808e-2_f64 * t35397 - 0.13903718850166016612e-2_f64 * t35400 - 0.3475929712541504153e-3_f64 * t35404 + 0.28048551634483599805e-4_f64 * t35406 - 0.50613927761474165061e-5_f64 * t35409 - 0.29524791194193262952e-5_f64 * t35412;
    (t37352, t37354, t37356, t37370)
}
