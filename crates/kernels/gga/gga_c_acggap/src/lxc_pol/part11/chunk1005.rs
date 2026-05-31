//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1005/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1005<F: Float>(t2294: F, t7630: F, t31253: F, t527: F, t2299: F, t7610: F, t33823: F, t33827: F, t33831: F, t33835: F, t33840: F, t33842: F, t33844: F, t33847: F, t33852: F, t33853: F, t33857: F, t33860: F, t33861: F, t33863: F) -> F {
    let t33865 = t7630 * t2294;
    let t33867 = t31253 * t527;
    let t33869 = t7610 * t2299;
    let t33871 = F::cast_from(0.31448092289604152068e-2_f64) * t33823 - F::cast_from(0.47172138434406228102e-2_f64) * t33827 - F::cast_from(0.62896184579208304136e-3_f64) * t33831 - F::cast_from(0.94344276868812456204e-2_f64) * t33835 - t33840 - t33842 + t33844 + F::cast_from(0.15724046144802076034e-3_f64) * t33847 + t33852 + F::cast_from(0.20965394859736101378e-3_f64) * t33853 + F::cast_from(0.62896184579208304134e-3_f64) * t33857 + t33860 - F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t33861 - t33863 / F::cast_from(48.0_f64) + F::cast_from(0.25724410870841842184e-2_f64) * t33865 - F::cast_from(0.42874018118069736972e-3_f64) * t33867 + F::cast_from(0.7862023072401038017e-3_f64) * t33869;
    t33871
}
