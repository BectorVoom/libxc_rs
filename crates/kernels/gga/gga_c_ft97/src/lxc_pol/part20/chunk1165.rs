//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1165/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1165<F: Float>(t28137: F, t8392: F, t107954: F, t109823: F, t13922: F, t14127: F, t14159: F, t1901: F, t2413: F, t242: F, t24668: F, t2469: F, t24748: F, t24752: F, t24816: F, t27742: F, t28141: F, t28267: F, t28364: F, t446: F, t51990: F, t53942: F, t6947: F, t724: F, t729: F, t754: F, t762: F, t766: F, t9707: F, t98016: F, t98021: F, t98029: F) -> (F,) {
    let t111190 = 4.0 / 9.0 * t8392 * t28137;
    let t111198 = 2.0 / 3.0 * t446 * t729 * t2469 * t28267 + 2.0 / 3.0 * t446 * t729 * t762 * t27742 * t766 - 2.0 / 9.0 * t98016 - 2.0 / 9.0 * t98021 - 2.0 / 3.0 * t446 * t242 * t109823 - 4.0 / 3.0 * t1901 * t53942 * t28364 - 4.0 * t1901 * t9707 * t754 * t28141 - t446 * t724 * t6947 * t2413 / 9.0 + t98029 / 9.0 + 2.0 / 9.0 * t1901 * t14159 * t24748 + t1901 * t14159 * t24752 / 9.0 + 2.0 / 27.0 * t1901 * t51990 * t24816 + t111190 - 2.0 / 3.0 * t1901 * t14127 * t24668 * t13922 - t446 * t242 * t107954 / 3.0;
    (t111198,)
}
