//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 487/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk487<F: Float>(t1517: F, t1518: F, t1650: F, t1979: F, t509: F, t1153: F, t1516: F, t1991: F, t1995: F, t2018: F, t368: F, t545: F, t562: F, t86: F) -> (F, F, F) {
    let t2026 = t1517 * t1518 * t1650;
    let t2029 = t509 * t1979;
    let t2033 = F::cast_from(0.619125e-2_f64) * t2018 * t545 + F::cast_from(0.9286875e-2_f64) * t562 * t1991 - F::cast_from(0.619125e-2_f64) * t562 * t1995 - t1516 - F::cast_from(0.26531111111111111111e-1_f64) * t1153 * t2026 - F::cast_from(0.39796666666666666666e-1_f64) * t86 * t368 * t2029;
    (t2026, t2029, t2033)
}
