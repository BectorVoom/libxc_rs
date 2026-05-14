//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 637/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk637<F: Float>(t1685: F, t668: F, t72: F, t1562: F, t2131: F, t2295: F, t5016: F, t2034: F, t6355: F, t1679: F, t2157: F, t2150: F, t623: F, t739: F, t8997: F, t132: F, t577: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9064 = t1685 * t668;
    let t9065 = t72 * t9064;
    let t9069 = t1562 * t2131;
    let t9071 = t5016 * t2295;
    let t9073 = t6355 * t2034;
    let t9075 = t1679 * t2157;
    let t9077 = t623 * t2150;
    let t9079 = t739 * t8997;
    let t9081 = t577 * t132;
    (t9064, t9065, t9069, t9071, t9073, t9075, t9077, t9079, t9081)
}
