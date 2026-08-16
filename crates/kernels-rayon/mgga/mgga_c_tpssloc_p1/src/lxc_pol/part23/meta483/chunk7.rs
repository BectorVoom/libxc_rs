//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1470/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1470(t1174: f64, t15740: f64, t1653: f64, t22162: f64, t22185: f64, t22284: f64, t22299: f64, t3578: f64, t45119: f64, t45192: f64, t5005: f64, t52903: f64, t53079: f64, t53099: f64, t6192: f64, t6232: f64, t65545: f64, t65815: f64, t72815: f64, t72849: f64, t72857: f64, t72864: f64, t75836: f64, t974: f64) -> f64 {
    let t79214 = -19.0_f64 / 288.0_f64 * t65545 * t6232 + 5.0_f64 / 576.0_f64 * t5005 * t22185 + t72815 / 54.0_f64 - t52903 * t22284 / 72.0_f64 + t72849 / 1152.0_f64 - 5.0_f64 / 1944.0_f64 * t72857 - t45119 * t3578 * t22299 * t1653 / 1152.0_f64 + t72864 / 576.0_f64 - t1174 * t974 * t45192 * t75836 / 12.0_f64 - t15740 * t22162 / 384.0_f64 - t65815 * t6192 / 384.0_f64 + t53079 / 2592.0_f64 + t53099 / 2592.0_f64;
    t79214
}
