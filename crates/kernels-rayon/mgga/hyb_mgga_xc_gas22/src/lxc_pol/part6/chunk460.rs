//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 460/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk460(t2112: f64, t2132: f64, t173: f64, t178: f64, t180: f64, t181: f64, t2104: f64, t2124: f64, t747: f64, t751: f64, t172: f64, t184: f64, t2113: f64, t2116: f64, t740: f64, t742: f64, t756: f64) -> (f64, f64, f64) {
    let t2133 = t2132 * t2112;
    let t2144 = -2.0_f64 * t2124 * t2112 * t180 + t747 * t2104 * t180 / 2.0_f64 + t2133 * t180 / 4.0_f64 - 4.0_f64 * t2112 * t181 - t178 * t2112 * t180 - 4.0_f64 * t751 * t2104 - t173 * t2104 * t180;
    let t2147 = -t2113 * t180 / 2.0_f64 + 2.0_f64 * t2116 * t2112 - t742 * t2104 + 2.0_f64 * t2104 * t184 + 4.0_f64 * t740 * t756 + 2.0_f64 * t172 * t2144;
    (t2133, t2144, t2147)
}
