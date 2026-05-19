//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1113/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1113<F: Float>(t50: F, t11627: F, t1369: F, t14837: F, t1699: F, t1702: F, t19487: F, t238: F, t2868: F, t2876: F, t35: F, t4011: F, t5460: F, t5465: F, t5468: F, t595: F, t821: F, t830: F, t833: F, t893: F, zeta_threshold: F) -> F {
    let t51 = t50 <= zeta_threshold;
    let t19966 = piecewise3::<F>(t51, F::new(0.0), F::new(40.0) / F::new(81.0) * t11627 * t1699 * t830 + F::new(64.0) / F::new(27.0) * t4011 * t19487 - F::new(8.0) / F::new(27.0) * t5460 * t833 + F::new(32.0) / F::new(9.0) * t893 * t35 * t595 - F::new(16.0) / F::new(9.0) * t1369 * t821 + F::new(16.0) / F::new(3.0) * t1369 * t2868 - F::new(8.0) / F::new(27.0) * t2876 * t1702 * t830 + F::new(8.0) / F::new(9.0) * t893 * t5468 * t238 + F::new(4.0) / F::new(9.0) * t5465 * t833 - t14837);
    t19966
}
