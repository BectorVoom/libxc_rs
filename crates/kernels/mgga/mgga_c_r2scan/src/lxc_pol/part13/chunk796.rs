//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 796/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk796<F: Float>(t7007: F, t88: F, t41: F, t1234: F, t2266: F, t2854: F, t4791: F, t4794: F, t4798: F, t4806: F, t4992: F, t6794: F, t6966: F, t6970: F, t6973: F, t6975: F) -> (F, F) {
    let t7008 = t7007 * t88;
    let t7009 = t41 * t7008;
    let t7011 = t2266 * t2854 * t1234;
    let t7012 = F::new(3.0) * t7011;
    let t7013 = t6966 - t6970 - t4791 + t4794 + t4798 - t4806 - t6973 - F::new(0.2363e1) * t6794 + t6975 + t4992 - t7009 - t7012;
    (t7009, t7013)
}
