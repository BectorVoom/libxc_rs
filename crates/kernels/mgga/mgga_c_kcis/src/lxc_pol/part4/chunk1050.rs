//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1050/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1050<F: Float>(t1083: F, t2840: F, t4567: F, t1788: F, t752: F, t1154: F, t1155: F, t1791: F, t318: F, t86: F, t1110: F, t1115: F, t1143: F, t14252: F, t14274: F, t1757: F, t1761: F, t1780: F, t2429: F, t3289: F, t3299: F, t3304: F, t3308: F, t3372: F, t365: F, t4626: F, t4671: F, t5102: F, t5133: F) -> (F,) {
    let t14962 = t2840 * t1083;
    let t14963 = t14962 * t4567;
    let t14966 = t752 * t1788;
    let t14992 = t1154 * t1155;
    let t14996 = t86 * t318 * t1791;
    let t14998 = -0.88437037037037037036e-1 * t5133 * t14963 + 0.5895802469135802469e-2 * t14966 - 0.619125e-2 * t1780 * t3308 + 0.9286875e-2 * t3372 * t1757 + 0.1857375e-1 * t1143 * t4626 + 0.1857375e-1 * t5102 * t1110 - 0.123825e-1 * t5102 * t1115 + 0.9286875e-2 * t365 * t14252 - 0.619125e-2 * t3372 * t1761 - 0.123825e-1 * t1143 * t4671 + 0.9286875e-2 * t1780 * t3299 + 0.123825e-1 * t1780 * t3304 - 0.619125e-2 * t365 * t14274 + 0.46434375e-2 * t1780 * t3289 - 0.53062222222222222222e-1 * t2429 * t14992 + 0.88437037037037037037e-2 * t14996;
    (t14998,)
}
