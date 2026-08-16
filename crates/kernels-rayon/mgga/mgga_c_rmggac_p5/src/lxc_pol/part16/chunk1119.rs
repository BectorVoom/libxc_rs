//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1119/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1119(t1652: f64, t9530: f64, t11905: f64, t1356: f64, t2211: f64, t2463: f64, t30360: f64, t41774: f64, t43844: f64, t43850: f64, t47465: f64, t47471: f64, t47473: f64, t47478: f64, t47484: f64, t47487: f64, t47490: f64, t47493: f64, t47495: f64, t47500: f64, t47505: f64, t884: f64) -> (f64, f64) {
    let t49210 = t9530 * t1652;
    let t49220 = -0.638468998399467591e-4_f64 * t47465 - 0.23942587439980034662e-4_f64 * t47471 + 0.212822999466489197e-4_f64 * t47473 - 0.39726959900411316773e-3_f64 * t41774 - 0.14546486215597515589e0_f64 * t47478 + t43844 - 0.23948483403727617128e0_f64 * t884 * t2211 * t30360 + t43850 - 0.11974241701863808564e0_f64 * t11905 * t2463 + 0.79828278012425390428e-1_f64 * t1356 * t49210 - 0.39726959900411316773e-4_f64 * t47484 + 0.17961362552795712846e0_f64 * t47487 - 0.35922725105591425692e0_f64 * t47490 - 0.8980681276397856423e-1_f64 * t47493 + 0.638468998399467591e-4_f64 * t47495 + 0.638468998399467591e-4_f64 * t47500 + 0.638468998399467591e-4_f64 * t47505;
    (t49210, t49220)
}
