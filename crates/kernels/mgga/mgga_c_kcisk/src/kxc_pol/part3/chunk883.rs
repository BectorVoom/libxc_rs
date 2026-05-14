//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 883/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk883<F: Float>(t14343: F, t14396: F, t14554: F, t14598: F, t1459: F, t4513: F, t9517: F, t1555: F, t524: F, t1596: F, t4348: F, t4349: F, t544: F, t12819: F, t12822: F, t12834: F, t12836: F, t12838: F, t12842: F, t1557: F, t4324: F, t4347: F, t4351: F, t4514: F) -> (F, F, F) {
    let t14600 = t14343 + t14396 + t14554 + t14598;
    let t14601 = t1459 * t14600;
    let t14602 = t9517 * t4513;
    let t14607 = t1555 * t1555;
    let t14608 = 1.0 / t14607;
    let t14609 = t524 * t14608;
    let t14610 = t4348 * t1596;
    let t14612 = 1.0 / t4349 / t544;
    let t14613 = t14610 * t14612;
    let t14628 = 0.579e0 * t1557 * t14602 + 0.223494e0 * t4347 * t14602 - 0.43134342e-1 * t14609 * t14613 - 0.579e0 * t4324 * t4514 - 0.386e0 * t1557 * t14613 + 0.34822083333333333333e-2 * t12819 + 0.34822083333333333333e-2 * t12822 + 0.51588271604938271605e-2 * t12834 + 0.46429444444444444443e-2 * t12836 + 0.23214722222222222222e-2 * t12838 + 0.38691203703703703703e-2 * t12842 + 0.579e0 * t4324 * t4351;
    (t14601, t14613, t14628)
}
