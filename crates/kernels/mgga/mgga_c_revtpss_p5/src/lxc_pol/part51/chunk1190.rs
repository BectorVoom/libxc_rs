//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1190/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1190<F: Float>(t125362: F, t1937: F, t125365: F, t33602: F, t6993: F, t127369: F, t127371: F, t127373: F, t127375: F, t127378: F, t127384: F, t127385: F, t127393: F, t127395: F, t127397: F, t127399: F, t28030: F, t32316: F, t33903: F, t4248: F, t4292: F, t5787: F, t651: F, t670: F, t7007: F, t8557: F, t8565: F) -> F {
    let t127401 = t125362 * t1937;
    let t127403 = t125365 * t1937;
    let t127405 = t33602 * t6993;
    let t127409 = -F::cast_from(2.0_f64) * t33903 * t651 * t670 - F::cast_from(2.0_f64) * t4292 * t651 * t8557 - F::cast_from(4.0_f64) * t28030 * t7007 - F::cast_from(2.0_f64) * t32316 * t4248 + t5787 * t8565 - t127369 - t127371 - t127373 - t127375 - t127378 - t127384 - t127385 - F::cast_from(4.0_f64) * t127393 - F::cast_from(4.0_f64) * t127395 - F::cast_from(4.0_f64) * t127397 - F::cast_from(4.0_f64) * t127399 - F::cast_from(4.0_f64) * t127401 - F::cast_from(4.0_f64) * t127403 - F::cast_from(4.0_f64) * t127405;
    t127409
}
