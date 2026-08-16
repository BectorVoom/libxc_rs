//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 549/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk549<F: Float>(t14425: F, t13957: F, t8041: F, t1356: F, t14276: F, t14278: F, t14280: F, t2228: F, t36: F, t305: F, t664: F, t8264: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14426 = F::cast_from(0.11974241701863808564e0_f64) * t14425;
    let t14427 = t8041 * t13957;
    let t14428 = t1356 * t14427;
    let t14429 = F::cast_from(0.11974241701863808564e0_f64) * t14428;
    let t14431 = F::cast_from(0.20455996240684006298e-1_f64) * t14276;
    let t14432 = F::cast_from(0.2727466165424534173e-1_f64) * t14278;
    let t14433 = F::cast_from(0.13637330827122670865e-1_f64) * t14280;
    let t14438 = t2228 * t36;
    let t14439 = t305 * t14438;
    let t14440 = F::cast_from(0.14967802127329760705e-1_f64) * t14439;
    let t14441 = t8264 * t664;
    (t14426, t14427, t14429, t14431, t14432, t14433, t14438, t14440, t14441)
}
