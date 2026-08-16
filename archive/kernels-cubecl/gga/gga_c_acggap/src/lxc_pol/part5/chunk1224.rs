//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1224/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1224<F: Float>(t1775: F, t987: F, t1163: F, t1165: F, t1552: F, t1759: F, t879: F, t322: F, t6263: F, t1532: F, t3456: F, t3372: F, t6405: F) -> (F, F, F, F, F) {
    let t22383 = t987 * t1775;
    let t22388 = t1163 * t1165 * t1552 * t1759 * t879;
    let t22394 = t6263 * t322;
    let t22397 = t3456 * t1165 * t1532 * t22394;
    let t22399 = t3372 * t6405;
    (t22383, t22388, t22394, t22397, t22399)
}
