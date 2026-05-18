//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1215/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1215<F: Float>(t14078: F, t14081: F, t14085: F, t10243: F, t1240: F, t13448: F, t14062: F, t14065: F, t14070: F, t14075: F, t15168: F, t15611: F) -> F {
    let t15632 = F::new(0.23214722222222222222e-2) * t14078;
    let t15638 = F::new(0.30952962962962962962e-2) * t14081;
    let t15639 = F::new(0.15476481481481481481e-2) * t14085;
    let t15640 = -F::new(0.92858888888888888886e-2) * t13448 + F::new(0.17411041666666666666e-2) * t14062 - F::new(0.38691203703703703703e-3) * t14065 - F::new(0.51588271604938271604e-3) * t14070 + F::new(0.46429444444444444443e-2) * t14075 - t15632 - F::new(0.13345e0) * t1240 * t15611 + F::new(0.66725e-1) * t1240 * t15168 - F::new(0.41270617283950617284e-2) * t10243 - t15638 - t15639;
    t15640
}
