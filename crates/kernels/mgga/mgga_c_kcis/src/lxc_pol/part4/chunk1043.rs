//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1043/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1043<F: Float>(t14616: F, t5047: F, t14832: F, t10753: F, t5099: F, t41: F, t9545: F, t14611: F, t5076: F, t14110: F, t5181: F, t5180: F, t10799: F, t1813: F, t10707: F, t5062: F) -> (F, F, F, F, F, F) {
    let t14833 = t5047 * t14616;
    let t14834 = t14832 * t14833;
    let t14836 = t10753 * t5099;
    let t14838 = t41 * t9545;
    let t14839 = t14838 * t14611;
    let t14840 = t5076 * t14839;
    let t14842 = t5181 * t14110;
    let t14843 = t5180 * t14842;
    let t14845 = t10799 * t1813;
    let t14847 = t10707 * t5062;
    (t14834, t14836, t14840, t14843, t14845, t14847)
}
