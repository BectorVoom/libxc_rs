//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1429/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1429(t3333: f64, t3351: f64, t3374: f64, t3399: f64, t440: f64, t3256: f64, t3263: f64, t3266: f64, t1094: f64, t11189: f64, t11192: f64, t11275: f64, t3315: f64, t43970: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44142 = t3333 * t3333;
    let t44146 = t3351 * t3351;
    let t44154 = 1.0_f64 / t3399 / t3374;
    let t44155 = t440 * t44154;
    let t44159 = t3256 * t3263;
    let t44161 = 12.0_f64 * t44159 * t3266;
    let t44162 = t1094 * t11189;
    let t44164 = 0.3859675079686208416e3_f64 * t44162 * t11192;
    let t44167 = 0.57895126195293126241e3_f64 * t11275 * t43970 * t3315;
    (t44142, t44146, t44154, t44155, t44161, t44164, t44167)
}
