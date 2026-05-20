//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1848/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1848<F: Float>(t87071: F, t92516: F, t116: F, t117: F, t1916: F, t1918: F, t22633: F, t25055: F, t25063: F, t25066: F, t25069: F, t572: F, t573: F, t5801: F, t5883: F, t5920: F, t6941: F, t6945: F, t6948: F, t87051: F, t87237: F, param_d: F) -> (F, F) {
    let t92517 = t87071 + t92516;
    let t92552 = F::new(18.0) * t116 * t572 * t87237 + F::new(3.0) * t117 * t572 * t87051 + F::new(24.0) * t22633 * t572 * t5801 + F::new(36.0) * t572 * t5883 * t5920 + t573 * t92517 * param_d + F::new(24.0) * t1916 * t25063 + F::new(72.0) * t1916 * t25066 + F::new(12.0) * t1916 * t25069 + F::new(12.0) * t1918 * t25055 + F::new(36.0) * t6941 * t6945 + F::new(18.0) * t6941 * t6948;
    (t92517, t92552)
}
