//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2472/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2472<F: Float>(t11791: F, t3490: F, t11789: F, t1227: F, t248: F, t3252: F, t3248: F, t11877: F, t3576: F, t11647: F, t1203: F, t204: F, t486: F) -> (F, F, F, F, F, F) {
    let t44968 = t3490 * t11791;
    let t44972 = t1227 * t248 * t11789 * t3252;
    let t44976 = t1227 * t248 * t11789 * t3248;
    let t44996 = t11877 * t3576;
    let t45002 = t1203 * t11647;
    let t45017 = t204 * t486;
    (t44968, t44972, t44976, t44996, t45002, t45017)
}
