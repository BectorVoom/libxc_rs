//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 907/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk907<F: Float>(t15386: F, t31057: F, t35700: F, t1347: F, t7614: F, t1967: F, t8502: F, t1998: F, t5089: F, t1451: F, t7605: F, t1423: F, t7736: F, t30318: F, t542: F, t2327: F, t7630: F) -> (F, F, F, F, F, F, F, F) {
    let t35702 = t31057 * t15386 * t35700;
    let t35703 = 0.94344276868812456204e-3 * t35702;
    let t35709 = t7614 * t1347;
    let t35710 = 0.32012600194825403606e-1 * t35709;
    let t35722 = t1967 * t8502;
    let t35723 = 0.25724410870841842184e-2 * t35722;
    let t35733 = t1998 * t5089;
    let t35736 = t7605 * t1451;
    let t35737 = 0.34299214494455789578e-2 * t35736;
    let t35738 = t7736 * t1423;
    let t35740 = t30318 * t542;
    let t35744 = t7630 * t2327;
    (t35703, t35710, t35723, t35733, t35737, t35738, t35740, t35744)
}
