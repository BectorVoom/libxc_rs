//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2196/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2196<F: Float>(t3540: F, t3567: F, t11159: F, t11539: F, t1174: F, t374: F, t485: F, t486: F, t9697: F, t1090: F, t3493: F, t11786: F, t3490: F) -> (F, F, F, F, F) {
    let t45224 = t3567 * t3540;
    let t45227 = t1174 * t11539 * t11159;
    let t45250 = F::cast_from(7.0_f64) / F::cast_from(31104.0_f64) * t485 * t374 * t9697 * t486;
    let t45251 = t1090 * t3493;
    let t45256 = t3490 * t11786;
    (t45224, t45227, t45250, t45251, t45256)
}
