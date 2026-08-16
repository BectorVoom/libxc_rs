//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 978/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk978<F: Float>(t1651: F, t3526: F, t587: F, t7942: F, t3465: F, t661: F, t5522: F, t639: F, t10524: F, t2677: F, t10535: F, t7853: F) -> (F, F, F, F, F) {
    let t11037 = t1651 * t3526;
    let t11038 = t587 * t11037;
    let t11039 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t11038;
    let t11040 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t7942;
    let t11041 = t3465 * t661;
    let t11042 = t5522 * t11041;
    let t11044 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t639 * t11042;
    let t11045 = t2677 * t10524;
    let t11047 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t639 * t11045;
    let t11048 = t7853 * t10535;
    (t11039, t11040, t11044, t11047, t11048)
}
