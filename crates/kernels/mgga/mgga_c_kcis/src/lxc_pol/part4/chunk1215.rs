//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1215/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1215<F: Float>(t14078: F, t14081: F, t14085: F, t10243: F, t1240: F, t13448: F, t14062: F, t14065: F, t14070: F, t14075: F, t15168: F, t15611: F) -> F {
    let t15632 = F::cast_from(0.23214722222222222222e-2_f64) * t14078;
    let t15638 = F::cast_from(0.30952962962962962962e-2_f64) * t14081;
    let t15639 = F::cast_from(0.15476481481481481481e-2_f64) * t14085;
    let t15640 = -F::cast_from(0.92858888888888888886e-2_f64) * t13448 + F::cast_from(0.17411041666666666666e-2_f64) * t14062 - F::cast_from(0.38691203703703703703e-3_f64) * t14065 - F::cast_from(0.51588271604938271604e-3_f64) * t14070 + F::cast_from(0.46429444444444444443e-2_f64) * t14075 - t15632 - F::cast_from(0.13345e0_f64) * t1240 * t15611 + F::cast_from(0.66725e-1_f64) * t1240 * t15168 - F::cast_from(0.41270617283950617284e-2_f64) * t10243 - t15638 - t15639;
    t15640
}
