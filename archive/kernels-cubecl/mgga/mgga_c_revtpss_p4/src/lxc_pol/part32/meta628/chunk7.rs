//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2016/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2016<F: Float>(t103273: F, t103276: F, t103280: F, t103283: F, t106022: F, t106024: F, t95671: F, t98976: F, t98979: F, t99002: F, t99004: F, t99009: F) -> F {
    let t110393 = F::cast_from(0.10164000561857065645e-2_f64) * t106022 - F::cast_from(0.80031500487063509015e-1_f64) * t106024 - F::cast_from(0.14457274399185490173e-3_f64) * t98976 + F::cast_from(0.2032800112371413129e-4_f64) * t98979 + t103273 + t103276 - t103280 + F::cast_from(0.10841600599314203355e-2_f64) * t99002 - t95671 + t99004 + t103283 - F::cast_from(0.18140473443734395377e0_f64) * t99009;
    t110393
}
