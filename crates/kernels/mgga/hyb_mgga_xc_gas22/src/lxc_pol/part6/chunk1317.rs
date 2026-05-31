//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1317/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1317<F: Float>(t7: F, t10536: F, t10541: F, t1793: F, t1796: F, t1808: F, t20666: F, t2170: F, t24587: F, t2750: F, t28813: F, t3302: F, t3804: F, t3814: F, t457: F, t545: F, t5891: F, t6536: F, t8632: F, t9909: F, zeta_threshold: F) -> F {
    let t8 = t7 <= zeta_threshold;
    let t28834 = piecewise3::<F>(t8, F::cast_from(0.0_f64), F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t20666 * t3814 * t1808 - F::cast_from(224.0_f64) / F::cast_from(27.0_f64) * t8632 * t28813 - F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t10536 * t1796 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t2170 * t457 * t2750 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3302 * t1793 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t3302 * t5891 - F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t6536 * t3804 * t1808 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2170 * t9909 * t545 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t10541 * t1796 - t24587);
    t28834
}
