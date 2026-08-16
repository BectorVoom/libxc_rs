//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2454/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2454<F: Float>(t72: F, t9940: F, t245: F, t3951: F, t3964: F, t9732: F, t1353: F, t9994: F, t136: F, t4010: F, t220: F, t2482: F, t27: F, t9991: F) -> (F, F, F, F, F, F, F) {
    let t47247 = t9940 * t72;
    let t47248 = t47247 * t245;
    let t47262 = t3964 * t9732 * t3951;
    let t47264 = t9994 * t1353;
    let t47273 = t4010 * t136;
    let t47274 = t47273 * t220;
    let t47293 = t2482 * t9991 * t27;
    (t47247, t47248, t47262, t47264, t47273, t47274, t47293)
}
