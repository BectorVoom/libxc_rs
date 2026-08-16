//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 877/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk877<F: Float>(t1081: F, t7363: F, t1089: F, t1884: F, t2635: F, t567: F, t1074: F, t2788: F, t2647: F, t1075: F, t1082: F, t1097: F, t222: F, t2766: F, t2772: F, t2783: F, t2784: F, t2789: F, t2792: F, t2798: F, t2802: F, t2803: F, t2806: F, t2809: F, t2810: F, t7263: F, t7267: F, t7316: F, t7317: F, t7324: F, t7327: F, t7330: F, t7360: F) -> (F, F, F, F, F, F, F) {
    let t7364 = t7363 * t1081;
    let t7367 = t1884 * t1089;
    let t7374 = t567 * t2635;
    let t7378 = t1884 * t1074;
    let t7385 = t567 * t2788;
    let t7389 = t567 * t2647;
    let t7393 = F::cast_from(0.10254018858216406658e4_f64) * t7316 * t7317 - F::cast_from(6.0_f64) * t2772 * t1082 * t2783 + F::cast_from(0.51947577317044391277e2_f64) * t2809 * t7324 - F::cast_from(0.35089341735807877242e1_f64) * t2802 * t7327 + F::cast_from(0.35089341735807877242e1_f64) * t2809 * t7330 + F::cast_from(1.0_f64) * t1075 * t7360 + F::cast_from(6.0_f64) * t2789 * t7364 - t7263 - t7267 + F::cast_from(0.21687162600603479684e-1_f64) * t222 * t7367 * t1097 - F::cast_from(0.16265371950452609763e-1_f64) * t222 * t2798 * t2806 - F::cast_from(0.48159733137676571078e0_f64) * t222 * t7374 * t2810 + F::cast_from(0.68493333333333333332e-1_f64) * t222 * t7378 * t1082 - F::cast_from(0.51369999999999999999e-1_f64) * t222 * t2766 * t2784 - F::cast_from(0.16522625736956710527e1_f64) * t222 * t7385 * t2792 + F::cast_from(0.32530743900905219526e-1_f64) * t222 * t7389 * t2803;
    (t7364, t7367, t7374, t7378, t7385, t7389, t7393)
}
