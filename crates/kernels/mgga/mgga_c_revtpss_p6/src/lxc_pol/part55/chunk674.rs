//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 674/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk674<F: Float>(t1113: F, t1940: F, t1963: F, t2403: F, t33: F, t7087: F, t7091: F, t7200: F, t7207: F, t1936: F, t2322: F, t5523: F) -> (F, F, F) {
    let t7214 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t1963 * t7200 + t1940 * t7087 * t33 / F::cast_from(2.0_f64) - t1940 * t7091 * t7207 / F::cast_from(2.0_f64) + t1940 * t1963 * t1113 / F::cast_from(2.0_f64);
    let t7226 = F::cast_from(2.0_f64) * t2322 * t1936;
    let t7228 = F::cast_from(2.0_f64) * t5523 * t1936;
    (t7214, t7226, t7228)
}
