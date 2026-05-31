//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2011/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2011<F: Float>(t98964: F, t98968: F, t98972: F, t98976: F, t98979: F, t92963: F, t92966: F, t92969: F, t92971: F, t92979: F, t95666: F, t98970: F) -> F {
    let t103264 = F::cast_from(0.30488190661738479625e-3_f64) * t98964;
    let t103265 = F::cast_from(0.11433071498151929859e-2_f64) * t98968;
    let t103267 = F::cast_from(0.4065600224742826258e-3_f64) * t98972;
    let t103269 = F::cast_from(0.72286371995927450867e-4_f64) * t98976;
    let t103270 = F::cast_from(0.10164000561857065645e-4_f64) * t98979;
    let t103271 = F::cast_from(0.2032800112371413129e-4_f64) * t92963 - F::cast_from(0.14457274399185490174e-3_f64) * t92966 - F::cast_from(35.0_f64) / F::cast_from(54.0_f64) * t92969 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t92971 - t103264 - t103265 - F::cast_from(0.34299214494455789578e-2_f64) * t98970 - t103267 + t95666 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t92979 - t103269 + t103270;
    t103271
}
