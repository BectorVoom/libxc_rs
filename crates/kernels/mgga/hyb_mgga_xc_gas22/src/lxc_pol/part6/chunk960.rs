//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 960/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk960<F: Float>(t3386: F, t828: F, t1359: F, t2267: F, t2252: F, t3389: F, t2275: F, t3385: F, t1358: F, t6712: F, t8651: F, t6530: F, t6533: F, t6749: F, t8648: F, t8676: F) -> (F, F, F, F, F, F, F, F) {
    let t8785 = t3386 * t828;
    let t8788 = t1359 * t2267;
    let t8791 = t3389 * t2252;
    let t8794 = t3385 * t2275;
    let t8795 = t8794 * t828;
    let t8798 = t3389 * t2267;
    let t8801 = t1358 * t6712;
    let t8802 = t8801 * t2252;
    let t8808 = F::new(0.34246666666666666666e-1) * t8651;
    let t8810 = -t6749 + F::new(0.45662222222222222222e-1) * t6530 - F::new(0.17123333333333333333e-1) * t6533 + F::new(0.22831111111111111111e-1) * t8676 - t8808 + F::new(0.5137e-1) * t8648;
    (t8785, t8788, t8791, t8795, t8798, t8802, t8808, t8810)
}
