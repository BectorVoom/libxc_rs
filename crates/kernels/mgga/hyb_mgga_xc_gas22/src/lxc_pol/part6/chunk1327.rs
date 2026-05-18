//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1327/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1327<F: Float>(t28849: F, t28874: F, t28906: F, t28930: F, t3418: F, t847: F, t20703: F, t20706: F, t20904: F, t24556: F, t24559: F, t24562: F, t284: F, t28853: F, t28856: F, t28859: F) -> (F, F, F, F) {
    let t28932 = t28849 + t28874 + t28906 + t28930;
    let t28937 = t3418 * t3418;
    let t28949 = t847 * t3418;
    let t28962 = (t20904 - F::new(0.57685185185185185184e-1) * t20703 + F::new(0.12361111111111111111e-1) * t20706 - F::new(0.57685185185185185187e-1) * t24556 + F::new(0.49444444444444444446e-1) * t24559 - F::new(0.18541666666666666667e-1) * t24562 + F::new(0.12361111111111111111e-1) * t28859 - F::new(0.18541666666666666667e-1) * t28853 + F::new(0.278125e-1) * t28856) * t284;
    (t28932, t28937, t28949, t28962)
}
