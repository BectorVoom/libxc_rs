//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 710/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk710<F: Float>(t7: F, t132: F, t1112: F, t3616: F, t1179: F, t2680: F, t224: F, t3: F, t1793: F, t545: F, t1238: F, t2688: F, t341: F, t675: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t3617 = t3616 * t1112;
    let t3619 = t2680 * t1179;
    let t3622 = t224 * t3;
    let t3626 = piecewise3::<F>(t8, F::new(0.0), F::new(4.0) / F::new(9.0) * t3619 * t545 + F::new(8.0) / F::new(3.0) * t3622 * t1793);
    let t3627 = t2688 * t1238;
    let t3630 = t341 * t3;
    let t3634 = piecewise3::<F>(t133, F::new(0.0), F::new(4.0) / F::new(9.0) * t3627 * t675 - F::new(8.0) / F::new(3.0) * t3630 * t1793);
    (t3617, t3619, t3622, t3626, t3627, t3630, t3634)
}
