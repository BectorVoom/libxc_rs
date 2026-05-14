//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1087/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1087<F: Float>(t122806: F, t122809: F, t123124: F, t123129: F, t129530: F, t129531: F, t129533: F, t129534: F, t129540: F, t129552: F, t129566: F, t129580: F, t129585: F, t1456: F, t1458: F, t1914: F, t2038: F, t29490: F, t32910: F, t34490: F, t5790: F, t7691: F, t7700: F, t7940: F, t7956: F, t8776: F) -> (F,) {
    let t129589 = t123124 + t129530 + t129531 + t7691 * t7956 + t122809 + t123129 + t129533 + t129534 + t2038 * t29490 + t122806 + t1458 * (t129540 + t129552 + t129566 + t129580) + t7940 * t7700 + t129585 + t1456 * t34490 + t1914 * t32910 + t5790 * t8776;
    (t129589,)
}
