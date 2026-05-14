//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 972/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk972<F: Float>(t1967: F, t8502: F, t2001: F, t4932: F, t4552: F, t1998: F, t5089: F, t1451: F, t7605: F, t1423: F, t7736: F, t30318: F, t542: F, t4886: F, t2327: F, t7630: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35722 = t1967 * t8502;
    let t35724 = t2001 * t4932;
    let t35731 = t2001 * t4552;
    let t35733 = t1998 * t5089;
    let t35736 = t7605 * t1451;
    let t35738 = t7736 * t1423;
    let t35740 = t30318 * t542;
    let t35742 = t2001 * t4886;
    let t35744 = t7630 * t2327;
    (t35722, t35724, t35731, t35733, t35736, t35738, t35740, t35742, t35744)
}
