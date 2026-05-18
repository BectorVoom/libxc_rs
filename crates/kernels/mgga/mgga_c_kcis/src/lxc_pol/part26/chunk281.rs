//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 281/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk281<F: Float>(t1517: F, t1518: F, t833: F, t1455: F, t509: F, t1153: F, t1478: F, t1483: F, t1507: F, t1516: F, t368: F, t545: F, t562: F, t86: F) -> (F, F, F) {
    let t1520 = t1517 * t1518 * t833;
    let t1523 = t509 * t1455;
    let t1527 = F::new(0.619125e-2) * t1507 * t545 + F::new(0.9286875e-2) * t562 * t1478 - F::new(0.619125e-2) * t562 * t1483 - t1516 - F::new(0.26531111111111111111e-1) * t1153 * t1520 - F::new(0.39796666666666666666e-1) * t86 * t368 * t1523;
    (t1520, t1523, t1527)
}
