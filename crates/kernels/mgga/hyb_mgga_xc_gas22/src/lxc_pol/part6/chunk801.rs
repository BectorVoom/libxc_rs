//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 801/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk801<F: Float>(t7: F, t132: F, t4407: F, t4457: F, t224: F, t2680: F, t3804: F, t3814: F, t2688: F, t341: F, t3925: F, t3938: F, t259: F, zeta_threshold: F) -> (F, F) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t4458 = t4407 + t4457;
    let t4467 = piecewise3::<F>(t8, F::new(0.0), F::new(4.0) / F::new(9.0) * t2680 * t3814 + F::new(4.0) / F::new(3.0) * t224 * t3804);
    let t4473 = piecewise3::<F>(t133, F::new(0.0), F::new(4.0) / F::new(9.0) * t2688 * t3925 + F::new(4.0) / F::new(3.0) * t341 * t3938);
    let t4475 = (t4467 + t4473) * t259;
    (t4458, t4475)
}
