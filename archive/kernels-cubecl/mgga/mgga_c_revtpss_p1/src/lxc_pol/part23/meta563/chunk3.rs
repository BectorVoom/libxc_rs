//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2135/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2135<F: Float>(t5: F, t10309: F, t13272: F, t1497: F, t21663: F, t2247: F, t22648: F, t22656: F, t22659: F, t22742: F, t4173: F, t5816: F, t5872: F, t603: F, t91: F) -> F {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t22746 = piecewise3::<F>(t8, F::cast_from(0.0_f64), -F::cast_from(120.0_f64) * t10309 * t22656 + F::cast_from(60.0_f64) * t13272 * t5816 - F::cast_from(12.0_f64) * t1497 * t21663 + F::cast_from(60.0_f64) * t2247 * t22659 + t22648 * t91 - F::cast_from(4.0_f64) * t22742 * t603 - F::cast_from(12.0_f64) * t4173 * t5872);
    t22746
}
