//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3142/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3142<F: Float>(t422: F, t57883: F, t57904: F, t1189: F, t1196: F, t17150: F, t3495: F, t57820: F, t57822: F, t57825: F, t57827: F, t57829: F, t57831: F, t57833: F, t57835: F, t57837: F, t57840: F, t57842: F, t57846: F, t57849: F, t57851: F, t57853: F, t57856: F, t57860: F, t57863: F) -> (F, F, F) {
    let t57907 = F::new(0.621814e-1) * (t57883 + t57904) * t422;
    let t57911 = F::cast_from(0.35089341735807877242e1_f64) * t1196 * t3495 * t17150 * t1189;
    let t57912 = -t57820 - t57822 - t57825 + t57827 - t57829 - t57831 - t57833 + t57835 + t57837 - t57840 + t57842 + t57846 + t57849 + t57851 + t57853 + t57856 + t57860 - t57863 - t57907 + t57911;
    (t57907, t57911, t57912)
}
