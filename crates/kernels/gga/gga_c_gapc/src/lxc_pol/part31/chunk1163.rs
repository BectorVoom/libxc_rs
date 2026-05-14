//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1163/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1163<F: Float>(t33209: F, t33214: F, t33221: F, t33226: F, t33230: F, t33232: F, t33240: F, t33242: F, t33245: F, t33248: F, t33252: F, t33254: F, t33259: F, t33263: F, t33265: F, t33270: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36508 = 0.13505639832369200846e-5 * t33209;
    let t36510 = 0.2025845974855380127e-5 * t33214;
    let t36512 = 0.16555927416768851825e-5 * t33221;
    let t36513 = 0.11984097313886885523e-6 * t33226;
    let t36515 = 0.18550690221634253912e-3 * t33230;
    let t36516 = 0.15458908518028544927e-5 * t33232;
    let t36517 = 0.24375961217880947793e-4 * t33240;
    let t36518 = 0.66295654499063700026e-7 * t33242;
    let t36520 = 0.2748593934505475288e-5 * t33245;
    let t36521 = 0.63350674672043801542e-5 * t33248;
    let t36522 = 0.63350674672043801542e-5 * t33252;
    let t36523 = 0.69504740211613770836e-3 * t33254;
    let t36524 = 0.17207124582926432293e-7 * t33259;
    let t36526 = 0.21720231316129303386e-4 * t33263;
    let t36527 = 0.43440462632258606772e-4 * t33265;
    let t36528 = 0.88465285289519332099e-6 * t33270;
    (t36508, t36510, t36512, t36513, t36515, t36516, t36517, t36518, t36520, t36521, t36522, t36523, t36524, t36526, t36527, t36528)
}
