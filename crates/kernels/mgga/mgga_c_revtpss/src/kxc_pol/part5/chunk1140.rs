//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1140/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1140<F: Float>(t1455: F, t1921: F, t571: F, t5808: F, t575: F, t6936: F, t5883: F, t648: F, t1501: F, t670: F, t6765: F, t1843: F, t4292: F) -> (F, F, F, F, F, F, F) {
    let t18184 = F::cast_from(2.0_f64) * t1455 * t1921;
    let t18186 = F::cast_from(2.0_f64) * t571 * t5808;
    let t18219 = t6936 * t575;
    let t18220 = t648 * t5883;
    let t18227 = t1501 * t670;
    let t18232 = t6765 * t670;
    let t18235 = t1843 * t4292;
    (t18184, t18186, t18219, t18220, t18227, t18232, t18235)
}
