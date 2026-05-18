//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1014/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1014<F: Float>(t1579: F, t2095: F, t355: F, t31477: F, t171: F, t5011: F, t31479: F, t2310: F, t7780: F, t31643: F, t527: F, t1418: F, t7605: F) -> (F, F, F, F, F, F, F) {
    let t35646 = t2095 * t1579 * t355;
    let t35648 = F::new(0.13073958333333333333e0) * t31477;
    let t35649 = t171 * t5011;
    let t35653 = F::new(0.13208198761633743869e-1) * t31479;
    let t35662 = t7780 * t2310;
    let t35664 = t31643 * t527;
    let t35672 = t7605 * t1418;
    (t35646, t35648, t35649, t35653, t35662, t35664, t35672)
}
