//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2017/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2017<F: Float>(t99099: F, t99102: F, t99113: F, t93067: F, t93069: F, t93073: F, t93077: F, t93080: F, t93084: F, t93086: F, t93088: F, t93091: F, t93095: F) -> F {
    let t103336 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t99099;
    let t103337 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t99102;
    let t103347 = F::cast_from(0.18071592998981862717e-4_f64) * t99113;
    let t103349 = t103336 - t103337 - F::cast_from(0.18140473443734395377e0_f64) * t93067 + F::cast_from(0.16006300097412701803e-1_f64) * t93069 + F::cast_from(0.43366402397256813418e-2_f64) * t93073 - F::cast_from(0.2032800112371413129e-3_f64) * t93077 + F::cast_from(0.28582678745379824648e-4_f64) * t93080 - F::cast_from(0.57165357490759649296e-4_f64) * t93084 - F::cast_from(0.80031500487063509015e-1_f64) * t93086 - F::cast_from(0.6097638132347695925e-3_f64) * t93088 + F::cast_from(0.28582678745379824648e-4_f64) * t93091 - t103347 + F::cast_from(0.10164000561857065645e-2_f64) * t93095;
    t103349
}
