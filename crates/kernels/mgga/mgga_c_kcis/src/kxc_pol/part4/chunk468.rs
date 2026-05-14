//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 468/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk468<F: Float>(t1517: F, t1518: F, t1650: F, t1979: F, t509: F, t1153: F, t1516: F, t1991: F, t1995: F, t2018: F, t368: F, t545: F, t562: F, t86: F, t552: F, t573: F, sigma2: F) -> (F, F, F, F, F) {
    let t2026 = t1517 * t1518 * t1650;
    let t2029 = t509 * t1979;
    let t2033 = 0.619125e-2 * t2018 * t545 + 0.9286875e-2 * t562 * t1991 - 0.619125e-2 * t562 * t1995 - t1516 - 0.26531111111111111111e-1 * t1153 * t2026 - 0.39796666666666666666e-1 * t86 * t368 * t2029;
    let t2034 = t2033 * t552;
    let t2035 = t2034 * sigma2;
    let t2036 = t2035 * t573;
    (t2026, t2029, t2033, t2035, t2036)
}
