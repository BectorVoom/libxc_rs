//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1429;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1430;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta382(t3333: f64, t3351: f64, t3374: f64, t3399: f64, t440: f64, t3256: f64, t3263: f64, t3266: f64, t1094: f64, t11189: f64, t11192: f64, t11275: f64, t3315: f64, t43970: f64, t3395: f64, t1124: f64, t11349: f64, t3355: f64, t427: f64, t3358: f64, t11176: f64, t1147: f64, t3368: f64, t3400: f64, t11285: f64, t11300: f64, t11307: f64, t11353: f64, t11356: f64, t11361: f64, t11365: f64, t1137: f64, t11400: f64, t11415: f64, t11420: f64, t1156: f64, t1157: f64, t3332: f64, t3357: f64, t3359: f64, t3371: f64, t3396: f64, t3401: f64, t3403: f64, t3404: f64, t43679: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t44142, t44146, t44154, t44155, t44161, t44164, t44167) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1429(t3333, t3351, t3374, t3399, t440, t3256, t3263, t3266, t1094, t11189, t11192, t11275, t3315, t43970);
        let (t44168, t44198) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1430(t3395, t1124, t11349, t3355, t427, t3358, t11176, t1147, t3368, t3400, t11285, t11300, t11307, t11353, t11356, t11361, t11365, t1137, t11400, t11415, t11420, t1156, t1157, t3332, t3357, t3359, t3371, t3396, t3401, t3403, t3404, t43679, t44142, t44146, t44155, t44161, t44164, t44167);
    (t44142, t44154, t44161, t44164, t44167, t44168, t44198)
}
