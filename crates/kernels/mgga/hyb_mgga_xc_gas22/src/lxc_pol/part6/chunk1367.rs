//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1367/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1367<F: Float>(t132: F, t1793: F, t27894: F, t10325: F, t10900: F, t10905: F, t2002: F, t2028: F, t21402: F, t2460: F, t25245: F, t2750: F, t3463: F, t3925: F, t3938: F, t457: F, t5891: F, t675: F, t6975: F, t9013: F, zeta_threshold: F) -> (F, F) {
    let t133 = t132 <= zeta_threshold;
    let t29765 = t27894 * t1793;
    let t29786 = piecewise3::<F>(t133, F::cast_from(0.0_f64), F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t21402 * t3925 * t2028 + F::cast_from(224.0_f64) / F::cast_from(27.0_f64) * t9013 * t29765 - F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t10900 * t2002 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t2460 * t457 * t2750 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3463 * t1793 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t3463 * t5891 - F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t6975 * t3938 * t2028 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2460 * t10325 * t675 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t10905 * t2002 + t25245);
    (t29765, t29786)
}
