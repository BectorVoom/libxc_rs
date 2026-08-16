//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1153/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1153<F: Float>(t311: F, t34159: F, t7089: F, t919: F, t2415: F, t3439: F, t9756: F, t1086: F, t11790: F, t23104: F, t11449: F, t11805: F, t190: F, t761: F) -> (F, F, F, F) {
    let t34235 = t311 * t7089 * t34159 * t919;
    let t34238 = t9756 * t2415 * t3439;
    let t34241 = t11790 * t1086 * t23104;
    let t34245 = t761 * t190 * t11449 * t11805;
    (t34235, t34238, t34241, t34245)
}
