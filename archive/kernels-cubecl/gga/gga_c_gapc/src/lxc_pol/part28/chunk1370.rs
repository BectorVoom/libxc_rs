//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1370/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1370<F: Float>(t33214: F, t33221: F, t33226: F, t33230: F, t33232: F, t33240: F, t33242: F, t33212: F, t33217: F, t33228: F, t36508: F, t33245: F) -> (F, F) {
    let t36510 = F::cast_from(0.2025845974855380127e-5_f64) * t33214;
    let t36512 = F::cast_from(0.16555927416768851825e-5_f64) * t33221;
    let t36513 = F::cast_from(0.11984097313886885523e-6_f64) * t33226;
    let t36515 = F::cast_from(0.18550690221634253912e-3_f64) * t33230;
    let t36516 = F::cast_from(0.15458908518028544927e-5_f64) * t33232;
    let t36517 = F::cast_from(0.24375961217880947793e-4_f64) * t33240;
    let t36518 = F::cast_from(0.66295654499063700026e-7_f64) * t33242;
    let t36519 = t36508 + F::cast_from(0.3623181683912940217e-6_f64) * t33212 - t36510 + F::cast_from(0.18115908419564701085e-6_f64) * t33217 + t36512 + t36513 - F::cast_from(0.25301106770833333336e-5_f64) * t33228 + t36515 + t36516 - t36517 - t36518;
    let t36520 = F::cast_from(0.2748593934505475288e-5_f64) * t33245;
    (t36519, t36520)
}
