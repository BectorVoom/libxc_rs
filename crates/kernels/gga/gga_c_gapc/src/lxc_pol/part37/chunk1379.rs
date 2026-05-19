//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1379/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1379<F: Float>(t33836: F, t33840: F, t33842: F, t33847: F, t33850: F, t33852: F, t33855: F, t33857: F, t33863: F, t33865: F, t33870: F, t33872: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36725 = F::cast_from(0.4637672555408563478e-4_f64) * t33836;
    let t36727 = F::cast_from(0.12141398358188788626e-5_f64) * t33840;
    let t36728 = F::cast_from(0.21587406280859666178e-5_f64) * t33842;
    let t36729 = F::cast_from(0.18477280112679442116e-5_f64) * t33847;
    let t36730 = F::cast_from(0.33764099580923002116e-6_f64) * t33850;
    let t36731 = F::cast_from(0.55982997132542680023e-7_f64) * t33852;
    let t36732 = F::cast_from(0.20220636637604418766e-5_f64) * t33855;
    let t36733 = F::cast_from(0.11594181388521408695e-4_f64) * t33857;
    let t36737 = F::cast_from(0.3077456993052877797e-8_f64) * t33863;
    let t36738 = F::cast_from(0.3077456993052877797e-8_f64) * t33865;
    let t36740 = F::cast_from(0.19336232562226912508e-7_f64) * t33870;
    let t36741 = F::cast_from(0.42205124476153752644e-7_f64) * t33872;
    (t36725, t36727, t36728, t36729, t36730, t36731, t36732, t36733, t36737, t36738, t36740, t36741)
}
