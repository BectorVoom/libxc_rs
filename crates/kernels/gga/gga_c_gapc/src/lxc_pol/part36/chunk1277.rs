//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1277/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1277<F: Float>(t1617: F, t3873: F, t4915: F, t10529: F, t10541: F, t15430: F, t3859: F, t35379: F, t35384: F, t35386: F, t35388: F, t35390: F, t35393: F, t35395: F, t35397: F, t35400: F, t35404: F, t35406: F, t35409: F, t35412: F) -> (F, F, F, F) {
    let t37352 = F::cast_from(6.0_f64) * t4915 * t3873 * t1617;
    let t37354 = F::cast_from(8.0_f64) * t10529 * t10541;
    let t37356 = F::cast_from(2.0_f64) * t15430 * t3859;
    let t37370 = F::cast_from(0.13903718850166016612e-2_f64) * t35379 - F::cast_from(0.49765421058075585109e-6_f64) * t35384 - F::cast_from(0.17379648562707520765e-3_f64) * t35386 - F::cast_from(0.3475929712541504153e-3_f64) * t35388 - F::cast_from(0.3475929712541504153e-2_f64) * t35390 - F::cast_from(0.9110506997065349711e-4_f64) * t35393 - F::cast_from(0.13903718850166016612e-2_f64) * t35395 + F::cast_from(0.10789865149347585808e-2_f64) * t35397 - F::cast_from(0.13903718850166016612e-2_f64) * t35400 - F::cast_from(0.3475929712541504153e-3_f64) * t35404 + F::cast_from(0.28048551634483599805e-4_f64) * t35406 - F::cast_from(0.50613927761474165061e-5_f64) * t35409 - F::cast_from(0.29524791194193262952e-5_f64) * t35412;
    (t37352, t37354, t37356, t37370)
}
