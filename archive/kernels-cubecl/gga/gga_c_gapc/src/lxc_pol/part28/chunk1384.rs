//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1384/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1384<F: Float>(t33831: F, t33836: F, t33840: F, t33842: F, t33847: F, t33850: F, t33852: F, t33855: F, t33857: F, t33834: F, t33838: F, t33863: F) -> (F, F) {
    let t36723 = F::cast_from(0.69504740211613770836e-3_f64) * t33831;
    let t36725 = F::cast_from(0.4637672555408563478e-4_f64) * t33836;
    let t36727 = F::cast_from(0.12141398358188788626e-5_f64) * t33840;
    let t36728 = F::cast_from(0.21587406280859666178e-5_f64) * t33842;
    let t36729 = F::cast_from(0.18477280112679442116e-5_f64) * t33847;
    let t36730 = F::cast_from(0.33764099580923002116e-6_f64) * t33850;
    let t36731 = F::cast_from(0.55982997132542680023e-7_f64) * t33852;
    let t36732 = F::cast_from(0.20220636637604418766e-5_f64) * t33855;
    let t36733 = F::cast_from(0.11594181388521408695e-4_f64) * t33857;
    let t36734 = -t36723 + F::cast_from(0.24457736545138888892e-4_f64) * t33834 - t36725 - F::cast_from(0.18115908419564701085e-6_f64) * t33838 + t36727 - t36728 + t36729 - t36730 + t36731 + t36732 - t36733;
    let t36737 = F::cast_from(0.3077456993052877797e-8_f64) * t33863;
    (t36734, t36737)
}
