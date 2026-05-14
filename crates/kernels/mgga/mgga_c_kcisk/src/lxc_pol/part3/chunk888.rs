//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 888/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk888<F: Float>(t13064: F, t325: F, t12885: F, t3725: F, t1212: F, t13099: F, t12884: F, t12888: F, t1528: F, t4428: F, t1524: F, t4460: F, t4459: F, t512: F, t507: F, t1536: F, t4437: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14736 = t325 * t13064;
    let t14737 = t12885 * t3725;
    let t14740 = t13099 * t1212;
    let t14743 = t325 * t12884;
    let t14744 = t12885 * t12888;
    let t14747 = t4428 * t1528;
    let t14752 = t1524 * t4460;
    let t14756 = 1.0 / t4459 / t512;
    let t14757 = t507 * t14756;
    let t14758 = t4437 * t1536;
    (t14736, t14737, t14740, t14743, t14744, t14747, t14752, t14757, t14758)
}
