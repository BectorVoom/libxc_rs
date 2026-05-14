//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1069/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1069<F: Float>(t39485: F, t39487: F, t39492: F, t39493: F, t39494: F, t41414: F, t41415: F, t41419: F, t41423: F, t43072: F, t43076: F, t43079: F, t43083: F, t43086: F, t43088: F, t43090: F, t43092: F, t43094: F, t43097: F, t43100: F, t43103: F, t43105: F, t43108: F, t43111: F) -> (F, F) {
    let t44216 = 0.18688645832733990742e0 * t39485 - t39487 - t39492 - t39493 - t39494 - 0.52396431978519890152e-1 * t43072 + t41414 - t41415 + t41419 - 0.43663693315433241794e-2 * t43076 + 0.46574606203128791246e-1 * t43079 + t41423;
    let t44229 = -0.20803732176130244552e1 * t43083 + 0.86682217400542685632e-1 * t43086 - 0.54878743191129263322e-1 * t43088 - 0.10975748638225852664e0 * t43090 + 0.10975748638225852664e0 * t43092 + 0.17336443480108537126e0 * t43094 - 0.95219938395347901947e-2 * t43097 + 0.47609969197673950973e-2 * t43100 - 0.28565981518604370584e-1 * t43103 + 0.95219938395347901947e-2 * t43105 - 0.10401866088065122276e1 * t43108 + 0.47609969197673950973e-2 * t43111;
    (t44216, t44229)
}
