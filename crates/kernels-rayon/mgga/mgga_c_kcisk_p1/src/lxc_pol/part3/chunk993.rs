//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 993/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk993(t14610: f64, t14612: f64, t12819: f64, t12822: f64, t12834: f64, t12836: f64, t12838: f64, t12842: f64, t14602: f64, t14609: f64, t1557: f64, t4324: f64, t4347: f64, t4351: f64, t4514: f64) -> (f64, f64) {
    let t14613 = t14610 * t14612;
    let t14628 = 0.579e0_f64 * t1557 * t14602 + 0.223494e0_f64 * t4347 * t14602 - 0.43134342e-1_f64 * t14609 * t14613 - 0.579e0_f64 * t4324 * t4514 - 0.386e0_f64 * t1557 * t14613 + 0.34822083333333333333e-2_f64 * t12819 + 0.34822083333333333333e-2_f64 * t12822 + 0.51588271604938271605e-2_f64 * t12834 + 0.46429444444444444443e-2_f64 * t12836 + 0.23214722222222222222e-2_f64 * t12838 + 0.38691203703703703703e-2_f64 * t12842 + 0.579e0_f64 * t4324 * t4351;
    (t14613, t14628)
}
