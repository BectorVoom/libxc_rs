//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3183/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3183<F: Float>(t12555: F, t5180: F, t1168: F, t12465: F, t12472: F, t12547: F, t12553: F, t16988: F, t3471: F, t3497: F, t3515: F, t3521: F, t435: F, t5120: F, t5184: F, t56260: F, t58468: F, t58472: F, t58475: F, t58477: F, t58479: F, t58481: F, t58591: F, t58592: F, t58598: F, t58618: F, t58639: F) -> F {
    let t58647 = t5180 * t12555;
    let t58654 = -t58468 + F::cast_from(1.0_f64) * t5120 * t12465 - t58472 - t58475 - t58477 - t58479 - t58481 - t58591 + F::cast_from(0.6207121550312808036e4_f64) * t58592 * t12472 * t3471 * t1168 - t58598 - F::cast_from(0.310907e-1_f64) * (t58618 + t58639) * t435 - F::cast_from(0.19751673498613801407e-1_f64) * t56260 + F::cast_from(0.51947577317044391277e2_f64) * t3521 * t16988 * t3515 + F::cast_from(0.30762056574649219973e4_f64) * t12553 * t58647 * t3497 + F::cast_from(0.17315859105681463759e2_f64) * t3521 * t5184 * t12547;
    t58654
}
