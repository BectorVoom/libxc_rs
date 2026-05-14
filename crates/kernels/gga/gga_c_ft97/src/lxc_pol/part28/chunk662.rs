//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 662/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk662<F: Float>(t32068: F, t32069: F, t379: F, t32067: F, t7240: F, t81: F, t7242: F, t432: F, t7165: F, t7238: F, t7239: F, t1307: F, t5617: F, t7243: F, t1800: F, t32058: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32071 = t32068 * t32069 * t379;
    let t32072 = t32067 * t32071;
    let t32075 = 1.0 / t7240 / t81;
    let t32076 = t32075 * t7242;
    let t32077 = t7165 * t432;
    let t32078 = t32076 * t32077;
    let t32080 = t7238 * t7239 * t32078;
    let t32082 = t1307 * t5617;
    let t32083 = t7243 * t32082;
    let t32085 = t7238 * t7239 * t32083;
    let t32087 = t1800 * t32058;
    (t32071, t32072, t32075, t32076, t32077, t32078, t32080, t32082, t32083, t32085, t32087)
}
