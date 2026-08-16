//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1041/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1041<F: Float>(t120006: F, t120151: F, t2453: F, t31798: F, t119974: F, t25304: F, t1035: F, t1052: F, t8514: F, t8515: F, t3058: F, t31991: F) -> (F, F, F, F, F, F) {
    let t120152 = t120151 * t120006;
    let t120154 = t2453 * t31798;
    let t120156 = F::cast_from(0.95199562775170587692e-3_f64) * t120154 * t119974;
    let t120157 = t25304 * t31798;
    let t120159 = F::cast_from(0.50779446784275991476e-2_f64) * t120157 * t119974;
    let t120179 = t1035 * t1052;
    let t120181 = t8514 * t8515 * t120179;
    let t120184 = t3058 * t31991;
    (t120152, t120156, t120159, t120179, t120181, t120184)
}
