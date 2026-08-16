//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1367/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1367(t33214: f64, t33221: f64, t33226: f64, t33230: f64, t33232: f64, t33240: f64, t33242: f64, t33212: f64, t33217: f64, t33228: f64, t36508: f64, t33245: f64) -> (f64, f64) {
    let t36510 = 0.2025845974855380127e-5_f64 * t33214;
    let t36512 = 0.16555927416768851825e-5_f64 * t33221;
    let t36513 = 0.11984097313886885523e-6_f64 * t33226;
    let t36515 = 0.18550690221634253912e-3_f64 * t33230;
    let t36516 = 0.15458908518028544927e-5_f64 * t33232;
    let t36517 = 0.24375961217880947793e-4_f64 * t33240;
    let t36518 = 0.66295654499063700026e-7_f64 * t33242;
    let t36519 = t36508 + 0.3623181683912940217e-6_f64 * t33212 - t36510 + 0.18115908419564701085e-6_f64 * t33217 + t36512 + t36513 - 0.25301106770833333336e-5_f64 * t33228 + t36515 + t36516 - t36517 - t36518;
    let t36520 = 0.2748593934505475288e-5_f64 * t33245;
    (t36519, t36520)
}
