//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1167/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1167<F: Float>(t5601: F, t9537: F, t6187: F, t1597: F, t4374: F, t27725: F, t6204: F, t1596: F, t2326: F, t32440: F) -> (F, F, F, F, F, F, F, F) {
    let t33826 = t9537 * t5601;
    let t33827 = t6187 * t33826;
    let t33830 = t4374 * t1597;
    let t33831 = t33830 * t27725;
    let t33832 = t6204 * t33831;
    let t33835 = t2326 * t1596;
    let t33836 = t32440 * t33835;
    let t33837 = t6204 * t33836;
    (t33826, t33827, t33830, t33831, t33832, t33835, t33836, t33837)
}
