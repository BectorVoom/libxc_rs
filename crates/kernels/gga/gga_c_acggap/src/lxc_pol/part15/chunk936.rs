//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 936/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk936<F: Float>(t35348: F, t35379: F, t35384: F, t35387: F, t35390: F, t35392: F, t35394: F, t35396: F, t35398: F, t35400: F, t35403: F, t35407: F, t35410: F, t35436: F, t35447: F, t35451: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37498 = 0.14291339372689912324e-2 * t35348;
    let t37519 = 0.62896184579208304138e-3 * t35379;
    let t37522 = 0.61125e-1 * t35384;
    let t37523 = t35387 / 4.0;
    let t37524 = t35390 / 16.0;
    let t37525 = 0.48018900292238105409e-1 * t35392;
    let t37526 = 0.13719685797782315831e-1 * t35394;
    let t37527 = 0.13719685797782315831e-1 * t35396;
    let t37528 = 0.68598428988911579156e-2 * t35398;
    let t37529 = 0.10289764348336736873e-1 * t35400;
    let t37531 = 0.34299214494455789578e-2 * t35403;
    let t37533 = t35407 / 16.0;
    let t37534 = t35410 / 48.0;
    let t37551 = 0.16006300097412701803e0 * t35436;
    let t37555 = 0.80031500487063509014e-2 * t35447;
    let t37557 = 0.64025200389650807212e-1 * t35451;
    (t37498, t37519, t37522, t37523, t37524, t37525, t37526, t37527, t37528, t37529, t37531, t37533, t37534, t37551, t37555, t37557)
}
