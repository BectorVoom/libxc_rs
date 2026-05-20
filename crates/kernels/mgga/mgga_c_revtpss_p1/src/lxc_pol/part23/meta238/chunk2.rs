//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1399/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1399<F: Float>(t3: F, t6936: F, t116: F, t5883: F, t117: F, t5920: F, t1916: F, t1918: F, t572: F, t573: F, t640: F, t76: F, param_d: F) -> (F, F, F, F, F, F) {
    let t6937 = t3 * t6936;
    let t6941 = param_d * t6936;
    let t6945 = t116 * t5883;
    let t6948 = t117 * t5920;
    let t6951 = F::new(6.0) * t1916 * t1918 + F::new(6.0) * t572 * t6945 + F::new(3.0) * t572 * t6948 + t573 * t6941;
    let t6977 = t76 * t640;
    (t6937, t6941, t6945, t6948, t6951, t6977)
}
