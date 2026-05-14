//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1175/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1175<F: Float>(t33831: F, t33836: F, t33840: F, t33842: F, t33847: F, t33850: F, t33852: F, t33855: F, t33857: F, t33834: F, t33838: F, t33863: F, t33865: F, t33870: F, t33872: F, t33875: F) -> (F, F, F, F, F, F) {
    let t36723 = 0.69504740211613770836e-3 * t33831;
    let t36725 = 0.4637672555408563478e-4 * t33836;
    let t36727 = 0.12141398358188788626e-5 * t33840;
    let t36728 = 0.21587406280859666178e-5 * t33842;
    let t36729 = 0.18477280112679442116e-5 * t33847;
    let t36730 = 0.33764099580923002116e-6 * t33850;
    let t36731 = 0.55982997132542680023e-7 * t33852;
    let t36732 = 0.20220636637604418766e-5 * t33855;
    let t36733 = 0.11594181388521408695e-4 * t33857;
    let t36734 = -t36723 + 0.24457736545138888892e-4 * t33834 - t36725 - 0.18115908419564701085e-6 * t33838 + t36727 - t36728 + t36729 - t36730 + t36731 + t36732 - t36733;
    let t36737 = 0.3077456993052877797e-8 * t33863;
    let t36738 = 0.3077456993052877797e-8 * t33865;
    let t36740 = 0.19336232562226912508e-7 * t33870;
    let t36741 = 0.42205124476153752644e-7 * t33872;
    let t36742 = 0.78582449132890172432e-8 * t33875;
    (t36734, t36737, t36738, t36740, t36741, t36742)
}
