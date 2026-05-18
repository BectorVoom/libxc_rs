//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 688/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk688<F: Float>(t132: F, t1238: F, t2460: F, t3: F, t937: F, t1793: F, t675: F, zeta_threshold: F) -> (F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t3463 = t2460 * t1238;
    let t3466 = t937 * t3;
    let t3470 = piecewise3::<f64>(t133, F::new(0.0), F::new(4.0) / F::new(9.0) * t3463 * t675 + F::new(2.0) / F::new(3.0) * t3466 * t1793);
    (t3463, t3466, t3470)
}
