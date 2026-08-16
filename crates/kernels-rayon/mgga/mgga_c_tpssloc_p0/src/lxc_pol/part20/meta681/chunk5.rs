//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2572/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2572(t3447: f64, t3451: f64, t52036: f64, t15357: f64, t3448: f64, t11579: f64, t11584: f64, t11593: f64, t15313: f64, t15320: f64, t15382: f64, t44517: f64, t44536: f64, t44540: f64, t44558: f64, t4900: f64, t4904: f64, t4908: f64, t4919: f64, t50857: f64, t50861: f64, t50873: f64, t50964: f64, t51995: f64, t52013: f64, t52019: f64, t52022: f64) -> f64 {
    let t52038 = t3447 * t52036 * t3451;
    let t52040 = t3448 * t15357;
    let t52047 = -0.16666666666666666666e-2_f64 * t51995 + 0.83333333333333333331e-3_f64 * t3447 * t15320 * t11579 + 0.16666666666666666666e-2_f64 * t3447 * t15320 * t11584 + 0.27777777777777777777e-3_f64 * t3447 * t4919 * t44540 + 0.16666666666666666666e-2_f64 * t3447 * t4919 * t44536 - 0.55555555555555555554e-3_f64 * t3447 * t4908 * t50873 - 0.83333333333333333331e-3_f64 * t52013 - 0.11111111111111111111e-2_f64 * t3447 * t44558 * t15382 + 0.11111111111111111111e-2_f64 * t52019 - 0.74074074074074074072e-3_f64 * t52022 + 0.66666666666666666665e-2_f64 * t3447 * t4900 * t50964 + 0.37037037037037037036e-3_f64 * t3447 * t4900 * t50857 + 0.13333333333333333332e-1_f64 * t3447 * t4900 * t50861 + 0.27777777777777777777e-3_f64 * t3447 * t44517 * t4904 + 0.55555555555555555554e-3_f64 * t52038 + 0.83333333333333333331e-3_f64 * t3447 * t52040 * t3451 + 0.83333333333333333331e-3_f64 * t3447 * t11593 * t15313;
    t52047
}
