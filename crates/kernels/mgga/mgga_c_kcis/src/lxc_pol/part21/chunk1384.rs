//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1384/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1384<F: Float>(t13093: F, t2167: F, t4527: F, t7671: F, t1658: F, t18401: F, t1876: F, t2169: F, t233: F, t27150: F, t2794: F, t2801: F, t28301: F, t441: F, t4534: F, t7827: F, t8027: F, t911: F, t91874: F, t91885: F, t91895: F, t91901: F, t92379: F) -> F {
    let t97548 = t13093 * t2167;
    let t97561 = F::cast_from(2.0_f64) * t4527 * t7671;
    let t97567 = -t91874 + t97548 - t233 * t4534 * t7827 / F::cast_from(8.0_f64) - t2169 * t2801 * t1876 / F::cast_from(16.0_f64) - t2794 * t8027 / F::cast_from(8.0_f64) - t91885 - t2169 * t18401 * t441 / F::cast_from(16.0_f64) + t97561 + t91895 - t91901 - t233 * t1658 * t27150 / F::cast_from(16.0_f64) + t92379 + t911 * t28301 / F::cast_from(8.0_f64);
    t97567
}
