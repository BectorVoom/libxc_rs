//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1090/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1090<F: Float>(t32682: F, t1944: F, t3437: F, t24745: F, t5539: F, t9647: F, t123: F, t24884: F, t2563: F, t10697: F, t7173: F, t11135: F, t5552: F, t2728: F, t8440: F, t16705: F, t3459: F) -> (F, F, F, F, F, F, F, F) {
    let t32683 = 0.85450291446024714264e-3 * t32682;
    let t32684 = t1944 * t3437;
    let t32685 = 0.99692006687028833308e-3 * t32684;
    let t32690 = t9647 * t5539 * t24745;
    let t32691 = 0.64087718584518535698e-3 * t32690;
    let t32692 = t24884 * t123;
    let t32694 = t9647 * t32692 * t2563;
    let t32695 = 0.19226315575355560709e-2 * t32694;
    let t32697 = t9647 * t10697 * t7173;
    let t32698 = 0.96131577876777803547e-3 * t32697;
    let t32708 = 4.0 * t5552 * t11135;
    let t32713 = 2.0 * t8440 * t2728;
    let t32715 = 2.0 * t16705 * t3459;
    (t32683, t32685, t32691, t32695, t32698, t32708, t32713, t32715)
}
