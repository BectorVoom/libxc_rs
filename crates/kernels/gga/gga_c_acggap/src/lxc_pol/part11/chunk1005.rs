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
    let t33871 = F::new(0.31448092289604152068e-2) * t33823 - F::new(0.47172138434406228102e-2) * t33827 - F::new(0.62896184579208304136e-3) * t33831 - F::new(0.94344276868812456204e-2) * t33835 - t33840 - t33842 + t33844 + F::new(0.15724046144802076034e-3) * t33847 + t33852 + F::new(0.20965394859736101378e-3) * t33853 + F::new(0.62896184579208304134e-3) * t33857 + t33860 - F::new(35.0) / F::new(432.0) * t33861 - t33863 / F::new(48.0) + F::new(0.25724410870841842184e-2) * t33865 - F::new(0.42874018118069736972e-3) * t33867 + F::new(0.7862023072401038017e-3) * t33869;
    t33871
}
