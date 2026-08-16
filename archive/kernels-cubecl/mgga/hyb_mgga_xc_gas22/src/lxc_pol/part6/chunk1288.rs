//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1288/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1288<F: Float>(t10450: F, t1289: F, t1318: F, t2004: F, t214: F, t2154: F, t23802: F, t23814: F, t23817: F, t23828: F, t23831: F, t23834: F, t23853: F, t23856: F, t23883: F, t23886: F, t23891: F, t23896: F, t23938: F, t23943: F, t2986: F, t3947: F, t3990: F, t684: F, t686: F, t766: F, t8456: F) -> F {
    let t27935 = t23802 / F::cast_from(72.0_f64) - F::cast_from(5.0_f64) / F::cast_from(432.0_f64) * t23814 - t23817 / F::cast_from(48.0_f64) + t23828 / F::cast_from(72.0_f64) - t23831 / F::cast_from(96.0_f64) - t23834 / F::cast_from(72.0_f64) - t23853 / F::cast_from(96.0_f64) + t23856 / F::cast_from(24.0_f64) + t684 * t2986 * t8456 * t1318 / F::cast_from(16.0_f64) + t684 * t2986 * t686 * t1289 * t214 / F::cast_from(16.0_f64) - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t2004 * t3947 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t10450 * t766 - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t3990 * t2154 - t23883 / F::cast_from(36.0_f64) - t23886 / F::cast_from(72.0_f64) - F::cast_from(7.0_f64) / F::cast_from(216.0_f64) * t23891 + t23896 / F::cast_from(18.0_f64) + t23938 / F::cast_from(24.0_f64) + t23943 / F::cast_from(54.0_f64);
    t27935
}
