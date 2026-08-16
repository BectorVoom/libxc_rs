//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2221/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2221<F: Float>(t1058: F, t27467: F, t100255: F, t100261: F, t100262: F, t100268: F, t100270: F, t100272: F, t15887: F, t16186: F, t1972: F, t25526: F, t3130: F, t375: F, t4797: F, t4869: F, t4875: F, t7122: F, t7125: F, t93764: F) -> F {
    let t100275 = F::cast_from(0.57165357490759649296e-3_f64) * t27467 * t1058;
    let t100282 = -F::cast_from(0.57165357490759649296e-3_f64) * t100255 * t3130 - F::cast_from(0.45732285992607719436e-2_f64) * t25526 * t4869 + t100261 - F::cast_from(0.76220476654346199061e-3_f64) * t100262 + F::cast_from(0.42874018118069736972e-3_f64) * t7122 * t16186 - F::cast_from(0.57165357490759649296e-3_f64) * t93764 * t4875 + F::cast_from(0.3811023832717309953e-3_f64) * t100268 - F::cast_from(0.30488190661738479624e-2_f64) * t100270 - F::cast_from(0.95275595817932748827e-4_f64) * t100272 + t100275 - F::cast_from(0.45732285992607719436e-2_f64) * t4797 * t7125 * t375 + F::cast_from(0.42874018118069736972e-3_f64) * t15887 * t1972 * t375;
    t100282
}
