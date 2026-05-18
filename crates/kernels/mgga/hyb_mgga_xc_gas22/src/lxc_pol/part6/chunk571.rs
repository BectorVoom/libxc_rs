//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 571/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk571<F: Float>(t132: F, t2002: F, t2028: F, t2688: F, t341: F, t259: F, t2686: F, zeta_threshold: F) -> F {
    let t133 = t132 <= zeta_threshold;
    let t2694 = piecewise3::<f64>(t133, F::new(0.0), F::new(4.0) / F::new(9.0) * t2688 * t2028 + F::new(4.0) / F::new(3.0) * t341 * t2002);
    let t2696 = (t2686 + t2694) * t259;
    t2696
}
