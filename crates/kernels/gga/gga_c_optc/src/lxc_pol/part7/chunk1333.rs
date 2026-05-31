//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1333/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1333<F: Float>(t1062: F, t8847: F, t2933: F, t2972: F, t393: F, t26261: F, t26264: F, t26252: F, t26258: F, t26268: F, t26271: F, t26326: F, t26328: F, t26330: F, t26332: F, t26347: F, t26351: F, t26354: F, t26358: F) -> (F, F, F) {
    let t26588 = t1062 * t8847;
    let t26593 = t393 / t2972 / t2933;
    let t26599 = F::cast_from(0.5356037037037037037e1_f64) * t26261;
    let t26600 = F::cast_from(0.16979925925925925926e1_f64) * t26264;
    let t26611 = F::cast_from(0.76514814814814814814e0_f64) * t26252 + F::cast_from(0.68863333333333333334e1_f64) * t26258 + t26599 + t26600 + F::cast_from(0.2366859375e0_f64) * t26268 + F::cast_from(0.94674375e0_f64) * t26271 + F::cast_from(0.3529725e1_f64) * t26347 - F::cast_from(0.13772666666666666666e1_f64) * t26326 - F::cast_from(0.91817777777777777776e0_f64) * t26328 - F::cast_from(0.55570666666666666668e0_f64) * t26351 + F::cast_from(0.55570666666666666666e0_f64) * t26354 + F::cast_from(0.27545333333333333333e1_f64) * t26330 + F::cast_from(0.21424148148148148148e1_f64) * t26332 + F::cast_from(0.12349037037037037037e1_f64) * t26358;
    (t26588, t26593, t26611)
}
