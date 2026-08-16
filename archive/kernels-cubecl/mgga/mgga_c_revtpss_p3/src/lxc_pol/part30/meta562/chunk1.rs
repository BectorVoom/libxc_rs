//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2008/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2008<F: Float>(t240: F, t25260: F, t10728: F, t2661: F, t2479: F, t25222: F, t25228: F, t9775: F, t10732: F, t25227: F, t10705: F, t25234: F) -> (F, F, F, F, F, F) {
    let t93082 = t25260 * t240;
    let t93084 = t2661 * t93082 * t10728;
    let t93086 = t25222 * t2479;
    let t93088 = t9775 * t25228;
    let t93091 = t2661 * t25227 * t10732;
    let t93095 = t25234 * t10705;
    (t93082, t93084, t93086, t93088, t93091, t93095)
}
