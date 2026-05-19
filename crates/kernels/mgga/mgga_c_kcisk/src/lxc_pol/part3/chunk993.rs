//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 993/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk993<F: Float>(t14610: F, t14612: F, t12819: F, t12822: F, t12834: F, t12836: F, t12838: F, t12842: F, t14602: F, t14609: F, t1557: F, t4324: F, t4347: F, t4351: F, t4514: F) -> (F, F) {
    let t14613 = t14610 * t14612;
    let t14628 = F::new(0.579e0) * t1557 * t14602 + F::new(0.223494e0) * t4347 * t14602 - F::new(0.43134342e-1) * t14609 * t14613 - F::new(0.579e0) * t4324 * t4514 - F::new(0.386e0) * t1557 * t14613 + F::cast_from(0.34822083333333333333e-2_f64) * t12819 + F::cast_from(0.34822083333333333333e-2_f64) * t12822 + F::cast_from(0.51588271604938271605e-2_f64) * t12834 + F::cast_from(0.46429444444444444443e-2_f64) * t12836 + F::cast_from(0.23214722222222222222e-2_f64) * t12838 + F::cast_from(0.38691203703703703703e-2_f64) * t12842 + F::new(0.579e0) * t4324 * t4351;
    (t14613, t14628)
}
