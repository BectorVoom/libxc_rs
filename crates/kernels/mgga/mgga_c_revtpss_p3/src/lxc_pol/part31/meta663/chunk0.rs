//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2245/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2245<F: Float>(t1937: F, t85360: F, t18245: F, t6993: F, t1448: F, t30122: F, t25082: F, t28197: F, t105886: F, t1312: F, t1936: F, t75439: F) -> (F, F, F, F, F) {
    let t109196 = F::cast_from(2.0_f64) * t85360 * t1937;
    let t109198 = F::cast_from(2.0_f64) * t18245 * t6993;
    let t109199 = t30122 * t1448;
    let t109202 = F::cast_from(12.0_f64) * t25082 * t28197 * t109199;
    let t109204 = F::cast_from(2.0_f64) * t1312 * t105886;
    let t109222 = F::cast_from(2.0_f64) * t75439 * t1936;
    (t109196, t109198, t109202, t109204, t109222)
}
