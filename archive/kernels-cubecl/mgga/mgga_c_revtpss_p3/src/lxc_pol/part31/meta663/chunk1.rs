//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2246/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2246<F: Float>(t1936: F, t85360: F, t18245: F, t7002: F, t109150: F, t109153: F, t105866: F, t108120: F, t109204: F, t109222: F, t1518: F, t21881: F, t25805: F, t28025: F, t28030: F, t33602: F, t4292: F, t5920: F, t670: F, t6985: F, t97622: F) -> F {
    let t109224 = F::cast_from(2.0_f64) * t85360 * t1936;
    let t109226 = F::cast_from(2.0_f64) * t18245 * t7002;
    let t109228 = F::cast_from(4.0_f64) * t109150 * t1936;
    let t109230 = F::cast_from(4.0_f64) * t109153 * t1936;
    let t109231 = F::cast_from(2.0_f64) * t105866 * t670 + F::cast_from(4.0_f64) * t108120 * t1518 + F::cast_from(4.0_f64) * t1518 * t97622 + F::cast_from(2.0_f64) * t21881 * t6985 + F::cast_from(2.0_f64) * t25805 * t5920 + F::cast_from(2.0_f64) * t28025 * t5920 + F::cast_from(4.0_f64) * t28030 * t4292 + F::cast_from(4.0_f64) * t33602 * t4292 + t109204 + t109222 + t109224 + t109226 + t109228 + t109230;
    t109231
}
