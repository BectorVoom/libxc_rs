//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1023/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1023<F: Float>(t14512: F, t417: F, t1245: F, t4967: F, t991: F, t1704: F, t2911: F, t9874: F, t2872: F, t4936: F, t1699: F, t9916: F, t14486: F, t14489: F, t14493: F, t14498: F, t14502: F, t1700: F, t4940: F, t4944: F, t4948: F, t9903: F) -> (F,) {
    let t14513 = t417 * t14512;
    let t14516 = t1245 * t4967;
    let t14518 = t991 * t14516 / 72.0;
    let t14522 = t9874 * t1704 * t2911;
    let t14523 = t417 * t14522;
    let t14527 = t2872 * t4936 / 162.0;
    let t14528 = t9916 * t1699;
    let t14529 = t991 * t14528;
    let t14531 = -t991 * t14486 / 144.0 + t991 * t14489 / 216.0 + 7.0 / 648.0 * t991 * t14493 + t991 * t14498 / 54.0 - t991 * t14502 / 288.0 + t2872 * t4944 / 54.0 + t2872 * t4948 / 27.0 - 2.0 / 81.0 * t2872 * t4940 + t991 * t14513 / 24.0 + t14518 + 11.0 / 324.0 * t9903 * t1700 - t991 * t14523 / 16.0 - t14527 - t14529 / 1296.0;
    (t14531,)
}
