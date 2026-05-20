//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2253/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2253<F: Float>(t20783: F, t26880: F, t5326: F, t8184: F, t20846: F, t26824: F, t29062: F, t5362: F, t1256: F, t30816: F, t104972: F, t112404: F, t1238: F, t20318: F, t26827: F, t29047: F, t29048: F, t29049: F, t29083: F, t5304: F, t6647: F, t97288: F, t97296: F) -> F {
    let t112468 = t26880 * t20783;
    let t112480 = t5326 * t8184;
    let t112483 = t26824 * t20846;
    let t112485 = t29062 * t5362;
    let t112487 = t30816 * t1256;
    let t112489 = F::cast_from(0.38110238327173099531e-3_f64) * t112468 - F::cast_from(0.5081365110289746604e-2_f64) * t29083 * t5304 - t104972 + F::cast_from(0.95275595817932748827e-4_f64) * t97288 + t97296 + t112404 * t29049 / F::new(27.0) - t29047 * t29048 * t20318 / F::new(144.0) - F::cast_from(0.42874018118069736972e-3_f64) * t26827 * t6647 + F::cast_from(0.45732285992607719436e-2_f64) * t112480 * t1238 + F::cast_from(0.57165357490759649296e-3_f64) * t112483 + F::cast_from(0.30488190661738479624e-2_f64) * t112485 + F::cast_from(0.96545937095505185473e-2_f64) * t112487;
    t112489
}
