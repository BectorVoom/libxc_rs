//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 460/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk460<F: Float>(t2112: F, t2132: F, t173: F, t178: F, t180: F, t181: F, t2104: F, t2124: F, t747: F, t751: F, t172: F, t184: F, t2113: F, t2116: F, t740: F, t742: F, t756: F) -> (F, F, F) {
    let t2133 = t2132 * t2112;
    let t2144 = -F::cast_from(2.0_f64) * t2124 * t2112 * t180 + t747 * t2104 * t180 / F::cast_from(2.0_f64) + t2133 * t180 / F::cast_from(4.0_f64) - F::cast_from(4.0_f64) * t2112 * t181 - t178 * t2112 * t180 - F::cast_from(4.0_f64) * t751 * t2104 - t173 * t2104 * t180;
    let t2147 = -t2113 * t180 / F::cast_from(2.0_f64) + F::cast_from(2.0_f64) * t2116 * t2112 - t742 * t2104 + F::cast_from(2.0_f64) * t2104 * t184 + F::cast_from(4.0_f64) * t740 * t756 + F::cast_from(2.0_f64) * t172 * t2144;
    (t2133, t2144, t2147)
}
