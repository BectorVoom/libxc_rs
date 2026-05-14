//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 899/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk899<F: Float>(t2016: F, t8622: F, t515: F, t7852: F, t2041: F, t4769: F, t2294: F, t7630: F, t31253: F, t527: F, t2299: F, t7610: F, t33823: F, t33827: F, t33831: F, t33835: F, t33840: F, t33842: F, t33844: F, t33847: F, t33852: F, t33853: F, t33857: F) -> (F,) {
    let t33859 = t2016 * t8622;
    let t33860 = 11.0 / 576.0 * t33859;
    let t33861 = t7852 * t515;
    let t33863 = t2041 * t4769;
    let t33865 = t7630 * t2294;
    let t33867 = t31253 * t527;
    let t33869 = t7610 * t2299;
    let t33871 = 0.31448092289604152068e-2 * t33823 - 0.47172138434406228102e-2 * t33827 - 0.62896184579208304136e-3 * t33831 - 0.94344276868812456204e-2 * t33835 - t33840 - t33842 + t33844 + 0.15724046144802076034e-3 * t33847 + t33852 + 0.20965394859736101378e-3 * t33853 + 0.62896184579208304134e-3 * t33857 + t33860 - 35.0 / 432.0 * t33861 - t33863 / 48.0 + 0.25724410870841842184e-2 * t33865 - 0.42874018118069736972e-3 * t33867 + 0.7862023072401038017e-3 * t33869;
    (t33871,)
}
