//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1291/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1291<F: Float>(t124169: F, t125948: F, t125950: F, t128898: F, t129370: F, t130932: F, t13426: F, t1519: F, t18227: F, t2108: F, t2163: F, t27060: F, t28652: F, t33287: F, t33306: F, t34399: F, t4248: F, t4257: F, t7537: F, t7683: F, t7969: F, t7988: F, t8892: F) -> F {
    let t131037 = -F::cast_from(2.0_f64) * t124169 * t1519 + t129370 * t2108 - F::cast_from(2.0_f64) * t130932 * t1519 - F::cast_from(2.0_f64) * t13426 * t8892 - F::cast_from(2.0_f64) * t18227 * t8892 - t2163 * t28652 - F::cast_from(2.0_f64) * t27060 * t7988 - F::cast_from(2.0_f64) * t33287 * t4257 - F::cast_from(2.0_f64) * t33306 * t4248 + t34399 * t7537 - t7683 * t7969 - t125948 - t125950 - t128898;
    t131037
}
