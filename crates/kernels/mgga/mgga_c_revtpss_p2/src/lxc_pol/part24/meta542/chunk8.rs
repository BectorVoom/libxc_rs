//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1601/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1601<F: Float>(t10309: F, t13272: F, t1497: F, t21663: F, t2247: F, t22656: F, t22659: F, t22742: F, t4173: F, t45972: F, t5816: F, t5872: F, t60224: F, t603: F, t60673: F, t85037: F, t87072: F, t87086: F, t87092: F, t87195: F, t87221: F, t91: F) -> F {
    let t87225 = t87072 * t91 - F::cast_from(16.0_f64) * t85037 * t1497 + F::cast_from(120.0_f64) * t60673 * t5816 - F::cast_from(24.0_f64) * t21663 * t5872 - F::cast_from(480.0_f64) * t60224 * t22656 + F::cast_from(240.0_f64) * t13272 * t22659 - F::cast_from(16.0_f64) * t4173 * t22742 + F::cast_from(840.0_f64) * t45972 * t87086 - F::cast_from(720.0_f64) * t10309 * t5816 * t5872 + F::cast_from(60.0_f64) * t2247 * t87092 + F::cast_from(80.0_f64) * t2247 * t1497 * t22742 - F::cast_from(4.0_f64) * t603 * (t87195 + t87221);
    t87225
}
