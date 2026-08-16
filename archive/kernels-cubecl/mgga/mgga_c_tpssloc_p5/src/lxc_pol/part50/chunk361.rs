//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 361/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk361<F: Float>(t1556: F, t913: F, t893: F, t1541: F, t917: F, t1548: F, t1551: F, t1554: F, t926: F, t929: F, t932: F, t936: F) -> (F, F, F, F, F, F) {
    let t1557 = t1556 * t913;
    let t1559 = F::cast_from(1.0_f64) * t893 * t1557;
    let t1561 = -t917 - F::cast_from(0.17123333333333333333e-1_f64) * t1541;
    let t1568 = F::cast_from(0.3529725e1_f64) * t1548 - t926 - F::cast_from(0.516475e0_f64) * t1541 + F::cast_from(0.6311625e0_f64) * t1551 - t929 - F::cast_from(0.104195e0_f64) * t1554;
    let t1569 = t1568 * t932;
    let t1573 = -t936 - F::cast_from(0.92708333333333333333e-2_f64) * t1541;
    (t1557, t1559, t1561, t1568, t1569, t1573)
}
