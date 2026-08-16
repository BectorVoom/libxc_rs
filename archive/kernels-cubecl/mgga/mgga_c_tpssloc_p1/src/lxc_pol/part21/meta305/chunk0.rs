//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1650/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1650<F: Float>(t1094: F, t3263: F, t3395: F, t3403: F, t11135: F, t11203: F, t135: F, t3477: F, t1174: F, t1176: F, t698: F) -> (F, F, F, F, F, F, F, F) {
    let t11424 = t1094 * t3263;
    let t11433 = t3395 * t3403;
    let t11444 = F::cast_from(0.53272592592592592592e-1_f64) * t11135;
    let t11459 = F::cast_from(0.55403703703703703703e-1_f64) * t11135;
    let t11487 = F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t11203;
    let t11513 = t135 * t3477;
    let t11514 = t1174 * t11513;
    let t11529 = t698 * t1176;
    (t11424, t11433, t11444, t11459, t11487, t11513, t11514, t11529)
}
