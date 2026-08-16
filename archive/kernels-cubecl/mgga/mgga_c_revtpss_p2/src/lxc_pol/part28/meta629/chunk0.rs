//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2265/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2265<F: Float>(t1936: F, t98484: F, t98487: F, t27123: F, t7002: F, t13514: F, t93: F, t101469: F, t1312: F, t28219: F, t25832: F, t7889: F) -> (F, F, F, F, F, F, F) {
    let t101517 = F::cast_from(2.0_f64) * t98484 * t1936;
    let t101519 = F::cast_from(4.0_f64) * t98487 * t1936;
    let t101521 = F::cast_from(4.0_f64) * t27123 * t7002;
    let t101522 = t93 * t13514;
    let t101524 = F::cast_from(2.0_f64) * t101522 * t1936;
    let t101526 = F::cast_from(2.0_f64) * t1312 * t101469;
    let t101528 = F::cast_from(4.0_f64) * t28219 * t7002;
    let t101530 = F::cast_from(2.0_f64) * t7889 * t25832;
    (t101517, t101519, t101521, t101524, t101526, t101528, t101530)
}
