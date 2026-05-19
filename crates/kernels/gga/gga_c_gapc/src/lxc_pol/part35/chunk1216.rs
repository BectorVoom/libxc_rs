//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1216/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1216<F: Float>(t11356: F, t26007: F, t9304: F, t2993: F, t3001: F, t33158: F, t35031: F, t35034: F, t35036: F, t35039: F, t35041: F, t35045: F, t35048: F, t35051: F, t35056: F) -> F {
    let t35059 = t9304 * t11356 * t26007;
    let t35062 = t2993 * t33158 * t3001;
    let t35064 = -F::cast_from(0.20241536458333333334e-4_f64) * t35031 - F::cast_from(0.2209926229259557733e-7_f64) * t35034 - F::cast_from(0.25340269868817520618e-3_f64) * t35036 - F::cast_from(0.12650960286458333334e-5_f64) * t35039 - F::cast_from(0.28985453471303521737e-5_f64) * t35041 - F::cast_from(0.19336854506021130164e-8_f64) * t35045 - F::cast_from(0.40483072916666666668e-4_f64) * t35048 - F::cast_from(0.49240895655712845849e-7_f64) * t35051 + F::cast_from(0.78584976712469872988e-8_f64) * t35056 + F::cast_from(0.21103240995305505364e-7_f64) * t35059 - F::cast_from(0.49522272202316919254e-5_f64) * t35062;
    t35064
}
