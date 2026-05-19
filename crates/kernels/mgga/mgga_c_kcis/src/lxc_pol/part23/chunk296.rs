//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 296/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk296<F: Float>(t1506: F, t1555: F, t1305: F, t1309: F, t597: F) -> (F, F, F, F, F) {
    let t1556 = t1506 * t1555;
    let t1557 = F::cast_from(0.17123333333333333333e-1_f64) * t1305;
    let t1559 = -t1557 - F::cast_from(0.17123333333333333333e-1_f64) * t1309;
    let t1562 = t597 * t597;
    let t1563 = F::new(1.0) / t1562;
    (t1556, t1557, t1559, t1562, t1563)
}
