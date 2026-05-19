//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1367/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1367<F: Float>(t33313: F, t33315: F, t33320: F, t33324: F, t33326: F, t33330: F, t33333: F, t33336: F, t33339: F, t33341: F, t33343: F, t33346: F, t33349: F) -> (F, F, F) {
    let t36542 = F::cast_from(0.1371666545474996961e-6_f64) * t33313;
    let t36543 = F::cast_from(0.3243554543208642639e-2_f64) * t33315;
    let t36556 = F::cast_from(0.43440462632258606772e-4_f64) * t33320 - F::cast_from(0.69504740211613770836e-3_f64) * t33324 - F::cast_from(0.3243554543208642639e-2_f64) * t33326 + F::cast_from(0.1433927048577202691e-8_f64) * t33330 - F::cast_from(0.2318836277704281739e-4_f64) * t33333 - F::cast_from(0.12290803273518880209e-8_f64) * t33336 + F::cast_from(0.16387737698025173612e-8_f64) * t33339 + F::cast_from(0.3243554543208642639e-2_f64) * t33341 - F::cast_from(0.61320337121513228211e-3_f64) * t33343 + F::cast_from(0.22466860691349365008e-6_f64) * t33346 + F::cast_from(0.11594181388521408695e-4_f64) * t33349;
    (t36542, t36543, t36556)
}
