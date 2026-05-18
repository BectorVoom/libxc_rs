//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 461/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk461<F: Float>(t7: F, t143: F, t172: F, t187: F, t2103: F, t2104: F, t2147: F, t740: F, t759: F, t139: F, t214: F, t26: F, t1796: F, t1885: F, t222: F, t226: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t144 = F::new(0.135e1) <= t143;
    let t2151 = piecewise3::<f64>(t144, t2103, -F::new(8.0) / F::new(3.0) * t2104 * t187 - F::new(16.0) / F::new(3.0) * t740 * t759 - F::new(8.0) / F::new(3.0) * t172 * t2147);
    let t2152 = t139 * t2151;
    let t2153 = t2152 * t214;
    let t2154 = t26 * t2153;
    let t2159 = piecewise3::<f64>(t8, F::new(0.0), t1796);
    let t2164 = t222 * t1885 * t226;
    (t2151, t2152, t2153, t2154, t2159, t2164)
}
