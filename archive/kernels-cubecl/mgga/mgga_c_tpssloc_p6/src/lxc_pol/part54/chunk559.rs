//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 559/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk559<F: Float>(t2645: F, t2647: F, t4181: F, t157: F, t2658: F, t1409: F, t184: F, t607: F, t1474: F, t172: F, t763: F, t185: F, t3966: F) -> (F, F, F, F) {
    let t4191 = t2645 * t4181 * t2647;
    let t4194 = t2658 * t157;
    let t4195 = t184 * t1409;
    let t4196 = t4195 * t607;
    let t4198 = F::cast_from(12.0_f64) * t4194 * t4196;
    let t4199 = t1474 * t172;
    let t4200 = t4199 * t763;
    let t4201 = F::cast_from(0.5848223622634646207e0_f64) * t4200;
    let t4202 = t185 * t3966;
    (t4191, t4198, t4201, t4202)
}
