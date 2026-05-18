//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1409/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1409<F: Float>(t7: F, t11192: F, t11197: F, t1793: F, t1796: F, t1808: F, t21896: F, t25907: F, t2680: F, t2750: F, t28813: F, t3619: F, t3804: F, t3814: F, t457: F, t545: F, t5891: F, t7281: F, t9340: F, t9909: F, zeta_threshold: F) -> F {
    let t8 = t7 <= zeta_threshold;
    let t30477 = piecewise3::<f64>(t8, F::new(0.0), F::new(40.0) / F::new(81.0) * t21896 * t3814 * t1808 - F::new(64.0) / F::new(27.0) * t9340 * t28813 - F::new(8.0) / F::new(27.0) * t11192 * t1796 + F::new(32.0) / F::new(9.0) * t2680 * t457 * t2750 + F::new(16.0) / F::new(9.0) * t3619 * t1793 - F::new(16.0) / F::new(3.0) * t3619 * t5891 - F::new(8.0) / F::new(27.0) * t7281 * t3804 * t1808 + F::new(8.0) / F::new(9.0) * t2680 * t9909 * t545 + F::new(4.0) / F::new(9.0) * t11197 * t1796 + t25907);
    t30477
}
