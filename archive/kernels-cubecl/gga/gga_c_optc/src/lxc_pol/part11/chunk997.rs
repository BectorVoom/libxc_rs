//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 997/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk997<F: Float>(t1188: F, t12943: F, t14849: F, t1588: F, t16095: F, t16097: F, t16099: F, t17733: F, t17746: F, t17750: F, t17753: F, t17853: F, t18178: F, t18184: F, t18188: F, t18191: F, t18194: F, t18200: F, t18205: F, t18214: F, t18218: F, t277: F, t4281: F, t4297: F, t490: F, t5229: F, t5246: F, t9254: F, t95: F) -> F {
    let t18223 = F::cast_from(0.25844881434903430496e-2_f64) * t95 * t277 * t18178 * t1188 - t17746 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4281 * t18184 - t4281 * t18188 + F::cast_from(200.0_f64) / F::cast_from(81.0_f64) * t4297 * t18191 - F::cast_from(50.0_f64) * t18194 * t1588 - F::cast_from(380000.0_f64) / F::cast_from(81.0_f64) * t18200 * t5246 + t17853 - F::cast_from(616.0_f64) / F::cast_from(27.0_f64) * t490 * t18205 - t17753 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16095 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t16097 + F::cast_from(100.0_f64) / F::cast_from(27.0_f64) * t14849 * t5229 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t16099 + F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t4297 * t18214 + t17750 - t17733 - t12943 / F::cast_from(3.0_f64) + F::cast_from(0.51689762869806860992e-2_f64) * t95 * t277 * t18218 * t9254;
    t18223
}
