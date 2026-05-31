//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 223/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk223<F: Float>(t1307: F, t1330: F, t26: F, t1309: F, t1320: F, t1322: F, t1325: F, t1329: F) -> (F, F, F) {
    let t1331 = t1330 * t1307;
    let t1332 = t26 * t1331;
    let t1334 = F::cast_from(0.1898925e1_f64) * t1320 - t1322 - F::cast_from(0.29896666666666666667e0_f64) * t1309 + F::cast_from(0.3071625e0_f64) * t1325 - t1329 - F::cast_from(0.82156666666666666667e-1_f64) * t1332;
    (t1331, t1332, t1334)
}
