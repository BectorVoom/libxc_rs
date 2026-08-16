//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2191/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2191<F: Float>(t1307: F, t1377: F, t22633: F, t22635: F, t6460: F, t1375: F, t1385: F, t16030: F, t1843: F, t22656: F, t22670: F, t26348: F, t26477: F, t28111: F, t28186: F, t28220: F, t3758: F, t3882: F, t3887: F, t5321: F, t5326: F, t6440: F, t7729: F, t90732: F, t91491: F) -> F {
    let t97705 = t22633 * t22635 * t1377 * t6460 * t1307;
    let t97717 = F::cast_from(4.0_f64) * t5321 * t26348 + F::cast_from(2.0_f64) * t1375 * t3887 * t28186 * t1385 + F::cast_from(4.0_f64) * t16030 * t7729 + F::cast_from(4.0_f64) * t26477 * t5326 + F::cast_from(2.0_f64) * t3758 * t28111 + F::cast_from(0.16449340668482264365e-1_f64) * t97705 - F::cast_from(2.0_f64) * t91491 * t1843 - F::cast_from(2.0_f64) * t90732 * t1843 + F::cast_from(4.0_f64) * t3882 * t28220 + F::cast_from(2.0_f64) * t22670 * t6440 + F::cast_from(2.0_f64) * t22656 * t6440;
    t97717
}
