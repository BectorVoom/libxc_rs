//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1313/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1313(t7: f64, t132: f64, t27905: f64, t27935: f64, t27990: f64, t28043: f64, t28084: f64, t28128: f64, t28645: f64, t28693: f64, t24480: f64, t10658: f64, t20895: f64, t2189: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t28697 = piecewise3(t134, 0.0_f64, t27905 + t27935 + t27990 + t28043 + t28084 + t28128 + t28645 + t28693);
    let t28698 = piecewise3(t8, 0.0_f64, t24480);
    let t28730 = 0.62071215503128080361e4_f64 * t20895 * t10658 * t2189;
    (t28697, t28698, t28730)
}
