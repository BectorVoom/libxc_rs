//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 966/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk966<F: Float>(t8688: F, t8691: F, t6691: F, t8670: F, t8673: F, t8676: F, t8683: F, t8685: F, t8695: F, t8699: F, t8703: F, t8706: F) -> (F, F, F) {
    let t8893 = F::new(0.41678e0) * t8688;
    let t8894 = F::new(0.41678e0) * t8691;
    let t8899 = F::cast_from(0.264729375e1_f64) * t8670 - F::cast_from(0.157790625e0_f64) * t8673 + F::cast_from(0.68863333333333333333e0_f64) * t8676 + F::new(0.3529725e1) * t8683 + F::new(0.6311625e0) * t8685 - t6691 - t8893 - t8894 + F::new(0.312585e0) * t8695 + F::new(0.62517e0) * t8699 + F::new(0.312585e0) * t8703 + F::cast_from(0.34731666666666666667e0_f64) * t8706;
    (t8893, t8894, t8899)
}
