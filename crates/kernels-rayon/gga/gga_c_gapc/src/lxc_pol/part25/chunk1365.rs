//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1365/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1365(t33214: f64, t33221: f64, t33226: f64, t33230: f64, t33232: f64, t33240: f64, t33242: f64, t33245: f64, t33248: f64, t33252: f64, t33254: f64, t33259: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36510 = 0.2025845974855380127e-5_f64 * t33214;
    let t36512 = 0.16555927416768851825e-5_f64 * t33221;
    let t36513 = 0.11984097313886885523e-6_f64 * t33226;
    let t36515 = 0.18550690221634253912e-3_f64 * t33230;
    let t36516 = 0.15458908518028544927e-5_f64 * t33232;
    let t36517 = 0.24375961217880947793e-4_f64 * t33240;
    let t36518 = 0.66295654499063700026e-7_f64 * t33242;
    let t36520 = 0.2748593934505475288e-5_f64 * t33245;
    let t36521 = 0.63350674672043801542e-5_f64 * t33248;
    let t36522 = 0.63350674672043801542e-5_f64 * t33252;
    let t36523 = 0.69504740211613770836e-3_f64 * t33254;
    let t36524 = 0.17207124582926432293e-7_f64 * t33259;
    (t36510, t36512, t36513, t36515, t36516, t36517, t36518, t36520, t36521, t36522, t36523, t36524)
}
