//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 501/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk501<F: Float>(t3860: F, t521: F, t1320: F, t1333: F, t123: F, t520: F, t2630: F, t1337: F, t2619: F, t514: F, t517: F, t1359: F, t2435: F) -> (F, F, F, F, F, F, F) {
    let t3862 = F::new(12.0) * t3860 * t521;
    let t3867 = F::new(8.0) * t1320 * t1333;
    let t3869 = t520 * t123;
    let t3871 = F::cast_from(0.10843581300301739842e-1_f64) * t3869 * t2630;
    let t3873 = F::cast_from(0.24415263074675393405e-3_f64) * t1337 * t2619;
    let t3874 = F::new(1.0) / t514;
    let t3881 = F::new(1.0) / t517;
    let t3894 = F::cast_from(0.73171657588172351096e-2_f64) * t2435 * t1359;
    (t3862, t3867, t3871, t3873, t3874, t3881, t3894)
}
