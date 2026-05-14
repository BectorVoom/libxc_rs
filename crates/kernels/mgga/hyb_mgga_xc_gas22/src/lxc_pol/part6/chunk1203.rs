//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1203/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1203<F: Float>(t10450: F, t1289: F, t1318: F, t2004: F, t214: F, t2154: F, t23802: F, t23814: F, t23817: F, t23828: F, t23831: F, t23834: F, t23853: F, t23856: F, t23883: F, t23886: F, t23891: F, t23896: F, t23938: F, t23943: F, t2986: F, t3947: F, t3990: F, t684: F, t686: F, t766: F, t8456: F) -> (F,) {
    let t27935 = t23802 / 72.0 - 5.0 / 432.0 * t23814 - t23817 / 48.0 + t23828 / 72.0 - t23831 / 96.0 - t23834 / 72.0 - t23853 / 96.0 + t23856 / 24.0 + t684 * t2986 * t8456 * t1318 / 16.0 + t684 * t2986 * t686 * t1289 * t214 / 16.0 - 3.0 / 32.0 * t2004 * t3947 - 3.0 / 32.0 * t10450 * t766 - 3.0 / 64.0 * t3990 * t2154 - t23883 / 36.0 - t23886 / 72.0 - 7.0 / 216.0 * t23891 + t23896 / 18.0 + t23938 / 24.0 + t23943 / 54.0;
    (t27935,)
}
