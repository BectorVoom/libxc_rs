//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2067/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2067<F: Float>(t4447: F, t92951: F, t14727: F, t25227: F, t2661: F, t4430: F, t93034: F, t14861: F, t1565: F, t93066: F, t25222: F, t4345: F) -> (F, F, F, F, F, F) {
    let t98991 = t92951 * t4447;
    let t98992 = F::cast_from(0.40015750243531754508e-2_f64) * t98991;
    let t99000 = t2661 * t25227 * t14727;
    let t99001 = F::cast_from(0.11433071498151929859e-3_f64) * t99000;
    let t99002 = t93034 * t4430;
    let t99006 = t2661 * t25227 * t14861;
    let t99007 = F::cast_from(0.28582678745379824648e-4_f64) * t99006;
    let t99009 = t93066 * t1565;
    let t99011 = t25222 * t4345;
    (t98992, t99001, t99002, t99007, t99009, t99011)
}
