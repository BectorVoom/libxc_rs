//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2340/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2340<F: Float>(t20994: F, t2563: F, t13251: F, t13262: F, t16816: F, t16836: F, t16845: F, t16893: F, t16969: F, t20908: F, t2623: F, t4178: F, t4180: F, t4182: F, t46875: F, t46876: F, t58705: F, t58709: F, t58723: F, t58731: F, t58735: F, t67607: F) -> F {
    let t67920 = t2563 * t20994;
    let t67926 = F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t16836 * t16845 + t13251 * t16969 / F::cast_from(128.0_f64) + t46875 + t16836 * t16893 / F::cast_from(512.0_f64) - F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t13262 * t4180 * t67607 * t16816 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t4178 * t4180 * t67607 * t4182 - F::cast_from(35.0_f64) / F::cast_from(192.0_f64) * t58705 - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t58709 - F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t58723 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t58731 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t67920 + F::cast_from(595.0_f64) / F::cast_from(3456.0_f64) * t46876 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t58735 - t2623 * t20908 / F::cast_from(768.0_f64);
    t67926
}
