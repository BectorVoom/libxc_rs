//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2051/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2051<F: Float>(t110058: F, t110102: F, t111089: F, t111130: F, t111174: F, t111214: F, t111260: F, t111301: F, t116: F, t30570: F, t109368: F, t117: F, t1459: F, t1916: F, t2113: F, t2115: F, t22544: F, t22559: F, t22565: F, t28975: F, t28981: F, t28987: F, t28990: F, t30654: F, t30657: F, t34359: F, t4292: F, t572: F, t573: F, t5795: F, t5802: F, t670: F, t6941: F, t6945: F, t7547: F, t7554: F, t8118: F, t8124: F, param_d: F) -> (F, F) {
    let t111304 = t110058 + t110102 + t111089 + t111130 + t111174 + t111214 + t111260 + t111301;
    let t111320 = t116 * t30570;
    let t111345 = F::new(3.0) * t109368 * t117 * t572 + t111304 * t573 * param_d + F::new(6.0) * t111320 * t572 * t670 + F::new(12.0) * t34359 * t4292 * t572 + F::new(12.0) * t1459 * t30654 + F::new(6.0) * t1459 * t30657 + F::new(12.0) * t1916 * t28975 + F::new(12.0) * t1916 * t28981 + F::new(12.0) * t1916 * t28987 + F::new(6.0) * t1916 * t28990 + F::new(12.0) * t2113 * t22559 + F::new(6.0) * t2113 * t22565 + F::new(3.0) * t2115 * t22544 + F::new(12.0) * t5795 * t8124 + F::new(12.0) * t5802 * t8118 + F::new(6.0) * t6941 * t7554 + F::new(6.0) * t6945 * t7547;
    (t111304, t111345)
}
