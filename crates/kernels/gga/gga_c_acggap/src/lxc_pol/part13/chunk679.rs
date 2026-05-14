//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 679/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk679<F: Float>(t409: F, t7685: F, t368: F, t4352: F, t7656: F, t598: F, t1089: F, t3300: F, t7679: F, t2100: F, t7676: F, t1988: F, t2092: F, t2090: F, t3201: F, t1083: F, t7533: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7686 = t7685 * t409;
    let t7689 = t4352 * t368 * t7656;
    let t7690 = t598 * t7689;
    let t7693 = t1089 * t3300 * t7679;
    let t7694 = t598 * t7693;
    let t7696 = t7676 * t2100;
    let t7697 = 0.18868855373762491241e-2 * t7696;
    let t7698 = t1988 * t2092;
    let t7699 = 0.42874018118069736972e-3 * t7698;
    let t7701 = t1089 * t3201 * t2090;
    let t7702 = t598 * t7701;
    let t7705 = t1089 * t1083 * t7533;
    (t7686, t7689, t7690, t7693, t7694, t7697, t7698, t7699, t7701, t7702, t7705)
}
