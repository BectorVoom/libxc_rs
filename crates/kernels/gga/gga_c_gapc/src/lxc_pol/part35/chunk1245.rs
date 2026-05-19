//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1245/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1245<F: Float>(t11270: F, t11273: F, t25530: F, t35379: F, t35384: F, t35386: F, t35388: F, t35390: F, t35393: F, t35395: F, t35397: F, t35400: F, t35404: F, t35406: F, t35409: F) -> F {
    let t35412 = t11270 * t25530 * t11273;
    let t35414 = F::cast_from(0.3475929712541504153e-3_f64) * t35379 - F::cast_from(0.12441355264518896277e-6_f64) * t35384 - F::cast_from(0.43449121406768801912e-4_f64) * t35386 - F::cast_from(0.86898242813537603824e-4_f64) * t35388 - F::cast_from(0.86898242813537603825e-3_f64) * t35390 - F::cast_from(0.22776267492663374277e-4_f64) * t35393 - F::cast_from(0.3475929712541504153e-3_f64) * t35395 + F::cast_from(0.2697466287336896452e-3_f64) * t35397 - F::cast_from(0.3475929712541504153e-3_f64) * t35400 - F::cast_from(0.86898242813537603824e-4_f64) * t35404 + F::cast_from(0.70121379086208999512e-5_f64) * t35406 - F::cast_from(0.12653481940368541265e-5_f64) * t35409 - F::cast_from(0.7381197798548315738e-6_f64) * t35412;
    t35414
}
