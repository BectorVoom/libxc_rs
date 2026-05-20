//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2632/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2632<F: Float>(t48396: F, t48419: F, t4010: F, t5591: F, t1353: F, t13716: F, t13892: F, t13902: F, t13910: F, t13911: F, t13914: F, t13917: F, t1392: F, t1394: F, t1395: F, t1412: F, t1879: F, t3829: F, t3889: F, t4050: F, t539: F, t5644: F, t5650: F, t5651: F, t9628: F, t9872: F) -> (F, F, F) {
    let t48421 = t48396 / F::new(2.0) + t48419 / F::new(2.0);
    let t48432 = t4010 * t5591;
    let t48436 = -F::new(36.0) * t1353 * t13716 * t1412 * t5650 - F::new(36.0) * t13910 * t3889 * t5650 + F::new(3.0) * t1394 * t48421 * t539 + F::new(180.0) * t3829 * t48432 * t5650 - F::new(12.0) * t5650 * t5651 * t9628 + F::new(9.0) * t13892 * t1395 - F::new(72.0) * t13902 * t13911 - F::new(36.0) * t13902 * t13914 + F::new(9.0) * t13917 * t1392 + F::new(3.0) * t1879 * t9872 - F::new(36.0) * t4050 * t5644;
    (t48421, t48432, t48436)
}
