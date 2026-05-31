//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1009/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1009<F: Float>(t33: F, t7086: F, t1113: F, t1940: F, t2403: F, t31859: F, t31863: F, t31876: F, t7091: F, t7200: F, t7207: F, t8490: F, t8494: F) -> F {
    let t32080 = t33 * t7086;
    let t32088 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t8490 * t7200 + t1940 * t31859 * t33 / F::cast_from(2.0_f64) - t1940 * t31863 * t7207 / F::cast_from(2.0_f64) + t1940 * t8490 * t1113 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t8494 * t7200 - t1940 * t7091 * t32080 + t1940 * t31876 * t7207 - t1940 * t8494 * t1113 / F::cast_from(2.0_f64);
    t32088
}
