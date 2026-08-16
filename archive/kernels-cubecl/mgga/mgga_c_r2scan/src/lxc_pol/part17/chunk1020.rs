//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1020/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1020<F: Float>(t11417: F, t11753: F, t11758: F, t11762: F, t11766: F, t11768: F, t11772: F, t11774: F, t12512: F, t12515: F, t12518: F, t12521: F, t12524: F, t12527: F, t12530: F) -> F {
    let t12798 = F::cast_from(0.17336443480108537126e0_f64) * t12512 - F::cast_from(0.39029762157531132074e-1_f64) * t11753 + F::cast_from(0.10975748638225852664e-1_f64) * t11758 + F::cast_from(0.93149212406257582492e-1_f64) * t11762 - F::cast_from(0.27944763721877274748e0_f64) * t11766 - F::cast_from(0.19514881078765566037e-1_f64) * t11768 - F::cast_from(0.93149212406257582492e-1_f64) * t11772 + F::cast_from(0.21951497276451705328e-1_f64) * t11774 - F::cast_from(0.17336443480108537126e0_f64) * t12515 - F::cast_from(0.86682217400542685632e-1_f64) * t12518 - F::cast_from(0.86682217400542685632e-1_f64) * t12521 - F::cast_from(0.54878743191129263322e-1_f64) * t12524 - F::cast_from(0.54878743191129263322e-1_f64) * t12527 - t11417 + F::cast_from(0.43663693315433241794e-2_f64) * t12530;
    t12798
}
