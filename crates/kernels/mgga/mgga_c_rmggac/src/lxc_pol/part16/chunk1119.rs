//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1119/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1119<F: Float>(t1652: F, t9530: F, t11905: F, t1356: F, t2211: F, t2463: F, t30360: F, t41774: F, t43844: F, t43850: F, t47465: F, t47471: F, t47473: F, t47478: F, t47484: F, t47487: F, t47490: F, t47493: F, t47495: F, t47500: F, t47505: F, t884: F) -> (F, F) {
    let t49210 = t9530 * t1652;
    let t49220 = -F::new(0.638468998399467591e-4) * t47465 - F::new(0.23942587439980034662e-4) * t47471 + F::new(0.212822999466489197e-4) * t47473 - F::new(0.39726959900411316773e-3) * t41774 - F::new(0.14546486215597515589e0) * t47478 + t43844 - F::new(0.23948483403727617128e0) * t884 * t2211 * t30360 + t43850 - F::new(0.11974241701863808564e0) * t11905 * t2463 + F::new(0.79828278012425390428e-1) * t1356 * t49210 - F::new(0.39726959900411316773e-4) * t47484 + F::new(0.17961362552795712846e0) * t47487 - F::new(0.35922725105591425692e0) * t47490 - F::new(0.8980681276397856423e-1) * t47493 + F::new(0.638468998399467591e-4) * t47495 + F::new(0.638468998399467591e-4) * t47500 + F::new(0.638468998399467591e-4) * t47505;
    (t49210, t49220)
}
