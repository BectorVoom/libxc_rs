//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 907/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk907<F: Float>(t1980: F, t35383: F, t7458: F, t31773: F, t8634: F, t2288: F, t4210: F, t15386: F, t31057: F, t1347: F, t7614: F, t31505: F, t31530: F, t31532: F, t1967: F, t8502: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35682 = t1980 * t7458 * t35383;
    let t35685 = t31773 * t8634;
    let t35700 = t2288 * t4210;
    let t35702 = t31057 * t15386 * t35700;
    let t35709 = t7614 * t1347;
    let t35713 = 0.18007087609589289529e-1 * t31505;
    let t35718 = 0.34299214494455789578e-2 * t31530;
    let t35719 = 0.34299214494455789578e-2 * t31532;
    let t35722 = t1967 * t8502;
    (t35682, t35685, t35700, t35702, t35709, t35713, t35718, t35719, t35722)
}
