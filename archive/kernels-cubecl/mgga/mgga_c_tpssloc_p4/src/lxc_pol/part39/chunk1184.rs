//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1184/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1184<F: Float>(t14841: F, t3404: F, t1164: F, t1098: F, t4737: F, t1119: F, t3308: F, t4740: F, t1657: F, t3312: F, t3316: F, t11282: F, t1694: F) -> (F, F, F, F, F) {
    let t14842 = t14841 * t3404;
    let t14844 = F::cast_from(0.10389515463408878255e3_f64) * t1164 * t14842;
    let t14845 = t4737 * t1098;
    let t14847 = F::cast_from(2.0_f64) * t14845 * t1119;
    let t14849 = F::cast_from(1.0_f64) * t4740 * t3308;
    let t14850 = t1657 * t3312;
    let t14852 = F::cast_from(0.16081979498692535067e2_f64) * t14850 * t3316;
    let t14853 = t11282 * t1694;
    (t14844, t14847, t14849, t14852, t14853)
}
