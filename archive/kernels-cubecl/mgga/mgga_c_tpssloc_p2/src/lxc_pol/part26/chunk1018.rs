//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1018/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1018<F: Float>(t11115: F, t11967: F, t510: F, t9416: F, t3696: F, t588: F, t592: F, t1285: F, t2223: F, t1287: F, t1291: F, t9874: F) -> (F, F, F, F, F, F, F) {
    let t11968 = t11115 + t11967;
    let t11972 = t510 * t9416;
    let t11975 = t588 * t3696;
    let t11976 = F::cast_from(12.0_f64) * t11975;
    let t11977 = t592 * t3696;
    let t11978 = F::cast_from(12.0_f64) * t11977;
    let t11979 = t2223 * t1285;
    let t11980 = F::cast_from(96.0_f64) * t11979;
    let t11981 = t2223 * t1287;
    let t11982 = F::cast_from(96.0_f64) * t11981;
    let t11984 = F::cast_from(0.56968947174242584612e-3_f64) * t1291 * t9874;
    (t11968, t11972, t11976, t11978, t11980, t11982, t11984)
}
