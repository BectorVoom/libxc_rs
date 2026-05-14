//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 564/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk564<F: Float>(t321: F, t3601: F, t123: F, t6118: F, t11627: F, t550: F, t5539: F, t11622: F, t1843: F, t10734: F, t10740: F, t10744: F, t10746: F, t10750: F, t1841: F, t3604: F, t3617: F, t650: F, t681: F) -> (F, F, F, F, F) {
    let t11679 = t321 * t3601;
    let t11680 = t11679 * t123;
    let t11681 = t11680 * t6118;
    let t11684 = t550 * t11627;
    let t11685 = t5539 * t11684;
    let t11688 = t550 * t11622;
    let t11689 = t1843 * t11688;
    let t11697 = 0.10254034973522965712e-1 * t650 * t3604 + 0.10254034973522965712e-1 * t650 * t3617 + 0.76905262301422242837e-2 * t681 * t3604 + 0.25635087433807414279e-2 * t1841 * t11681 - 0.17090058289204942852e-2 * t1841 * t11685 + 0.85450291446024714263e-3 * t1841 * t11689 + 0.17090058289204942853e-2 * t10734 - 0.1281754371690370714e-2 * t10740 - 0.1281754371690370714e-2 * t10744 + 0.1281754371690370714e-2 * t10746 + 0.1281754371690370714e-2 * t10750;
    (t11679, t11680, t11684, t11688, t11697)
}
