//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 889/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk889<F: Float>(t45902: F, t10914: F, t2365: F, t35446: F, t13555: F, t4614: F, t833: F, t10811: F, t10978: F, t3470: F, t37061: F, t36590: F, t955: F) -> (F, F, F, F, F, F) {
    let t45903 = F::cast_from(0.14896037479937677779e-1_f64) * t45902;
    let t45905 = t10914 * t2365 * t35446;
    let t45906 = F::cast_from(0.89376224879626066674e-1_f64) * t45905;
    let t45913 = F::cast_from(0.15337170381568299871e2_f64) * t833 * t4614 * t13555;
    let t45915 = F::cast_from(0.85801175884441024006e1_f64) * t10811 * t10978;
    let t45922 = F::cast_from(0.10725146985555128001e1_f64) * t37061 * t3470;
    let t45931 = F::cast_from(0.23833659967900284446e0_f64) * t955 * t36590;
    (t45903, t45906, t45913, t45915, t45922, t45931)
}
