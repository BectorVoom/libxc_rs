//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2116/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2116<F: Float>(t26024: F, t5661: F, t14054: F, t25986: F, t2661: F, t13874: F, t7271: F, t94477: F, t98211: F, t98213: F, t98215: F, t98217: F, t98218: F, t98220: F, t98222: F, t98224: F) -> F {
    let t98226 = t26024 * t5661;
    let t98227 = F::cast_from(0.40015750243531754508e-2_f64) * t98226;
    let t98229 = t2661 * t25986 * t14054;
    let t98230 = F::cast_from(0.11433071498151929859e-3_f64) * t98229;
    let t98231 = t7271 * t13874;
    let t98233 = F::cast_from(0.17149607247227894789e-2_f64) * t98211 - F::cast_from(0.42874018118069736972e-3_f64) * t98213 + F::cast_from(0.17149607247227894789e-2_f64) * t98215 - t94477 + t98217 - F::cast_from(0.60976381323476959249e-3_f64) * t98218 - F::cast_from(0.90357964994909313586e-5_f64) * t98220 - F::cast_from(0.80031500487063509016e-1_f64) * t98222 - F::cast_from(0.11337795902333997111e-1_f64) * t98224 + t98227 - t98230 + F::cast_from(0.85748036236139473945e-2_f64) * t98231;
    t98233
}
