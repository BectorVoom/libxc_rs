//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 877/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk877(t1081: f64, t7363: f64, t1089: f64, t1884: f64, t2635: f64, t567: f64, t1074: f64, t2788: f64, t2647: f64, t1075: f64, t1082: f64, t1097: f64, t222: f64, t2766: f64, t2772: f64, t2783: f64, t2784: f64, t2789: f64, t2792: f64, t2798: f64, t2802: f64, t2803: f64, t2806: f64, t2809: f64, t2810: f64, t7263: f64, t7267: f64, t7316: f64, t7317: f64, t7324: f64, t7327: f64, t7330: f64, t7360: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7364 = t7363 * t1081;
    let t7367 = t1884 * t1089;
    let t7374 = t567 * t2635;
    let t7378 = t1884 * t1074;
    let t7385 = t567 * t2788;
    let t7389 = t567 * t2647;
    let t7393 = 0.10254018858216406658e4_f64 * t7316 * t7317 - 6.0_f64 * t2772 * t1082 * t2783 + 0.51947577317044391277e2_f64 * t2809 * t7324 - 0.35089341735807877242e1_f64 * t2802 * t7327 + 0.35089341735807877242e1_f64 * t2809 * t7330 + 1.0_f64 * t1075 * t7360 + 6.0_f64 * t2789 * t7364 - t7263 - t7267 + 0.21687162600603479684e-1_f64 * t222 * t7367 * t1097 - 0.16265371950452609763e-1_f64 * t222 * t2798 * t2806 - 0.48159733137676571078e0_f64 * t222 * t7374 * t2810 + 0.68493333333333333332e-1_f64 * t222 * t7378 * t1082 - 0.51369999999999999999e-1_f64 * t222 * t2766 * t2784 - 0.16522625736956710527e1_f64 * t222 * t7385 * t2792 + 0.32530743900905219526e-1_f64 * t222 * t7389 * t2803;
    (t7364, t7367, t7374, t7378, t7385, t7389, t7393)
}
