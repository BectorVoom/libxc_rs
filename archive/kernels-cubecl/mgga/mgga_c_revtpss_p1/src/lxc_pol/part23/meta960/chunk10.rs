//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3243/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3243<F: Float>(t4245: F, t5883: F, t1310: F, t1502: F, t18220: F, t1843: F, t1911: F, t21658: F, t21881: F, t21882: F, t22525: F, t22639: F, t22747: F, t27123: F, t30138: F, t4246: F, t4248: F, t4292: F, t4293: F, t508: F, t5517: F, t5877: F, t5884: F, t5921: F, t651: F, t6765: F) -> (F, F) {
    let t85329 = t4245 * t5883;
    let t85343 = -F::cast_from(6.0_f64) * t1843 * t21881 * t651 - F::cast_from(6.0_f64) * t4292 * t651 * t6765 - F::cast_from(6.0_f64) * t1310 * t22639 - t1310 * t22747 - F::cast_from(3.0_f64) * t1502 * t21658 - F::cast_from(6.0_f64) * t18220 * t1843 + F::cast_from(3.0_f64) * t1911 * t22525 - F::cast_from(6.0_f64) * t21882 * t4248 - F::cast_from(6.0_f64) * t27123 * t5921 - F::cast_from(12.0_f64) * t30138 * t4293 - F::cast_from(3.0_f64) * t4246 * t6765 - F::cast_from(6.0_f64) * t508 * t85329 - F::cast_from(3.0_f64) * t5517 * t5877 - F::cast_from(6.0_f64) * t5517 * t5884;
    (t85329, t85343)
}
