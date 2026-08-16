//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1248/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1248<F: Float>(t2986: F, t4514: F, t61250: F, t13847: F, t17794: F, t17863: F, t48279: F, t10231: F, t21409: F, t973: F, t21462: F, t2970: F) -> (F, F, F, F, F) {
    let t69686 = t2986 * t61250 * t4514;
    let t69691 = t2986 * t13847 * t17794;
    let t69699 = t2986 * t48279 * t17863;
    let t69727 = t973 * t10231 * t21409;
    let t69739 = t973 * t2970 * t21462;
    (t69686, t69691, t69699, t69727, t69739)
}
