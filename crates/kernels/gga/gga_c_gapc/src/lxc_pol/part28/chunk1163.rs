//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1163/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1163<F: Float>(t33232: F, t33240: F, t33242: F, t33212: F, t33217: F, t33228: F, t36508: F, t36510: F, t36512: F, t36513: F, t36515: F, t33245: F, t33248: F, t33252: F, t33254: F, t33259: F) -> (F, F, F, F, F, F) {
    let t36516 = 0.15458908518028544927e-5 * t33232;
    let t36517 = 0.24375961217880947793e-4 * t33240;
    let t36518 = 0.66295654499063700026e-7 * t33242;
    let t36519 = t36508 + 0.3623181683912940217e-6 * t33212 - t36510 + 0.18115908419564701085e-6 * t33217 + t36512 + t36513 - 0.25301106770833333336e-5 * t33228 + t36515 + t36516 - t36517 - t36518;
    let t36520 = 0.2748593934505475288e-5 * t33245;
    let t36521 = 0.63350674672043801542e-5 * t33248;
    let t36522 = 0.63350674672043801542e-5 * t33252;
    let t36523 = 0.69504740211613770836e-3 * t33254;
    let t36524 = 0.17207124582926432293e-7 * t33259;
    (t36519, t36520, t36521, t36522, t36523, t36524)
}
