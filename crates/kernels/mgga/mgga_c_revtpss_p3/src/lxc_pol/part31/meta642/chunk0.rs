//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2099/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2099<F: Float>(t1459: F, t30188: F, t116: F, t30004: F, t572: F, t670: F, t1518: F, t1936: F, t4292: F, t6941: F, t7334: F, t30194: F) -> (F, F, F, F, F) {
    let t105818 = F::cast_from(12.0_f64) * t1459 * t30188;
    let t105819 = t116 * t30004;
    let t105822 = F::cast_from(6.0_f64) * t572 * t105819 * t670;
    let t105823 = t1518 * t1936;
    let t105826 = F::cast_from(12.0_f64) * t572 * t105823 * t4292;
    let t105830 = F::cast_from(3.0_f64) * t6941 * t7334;
    let t105834 = F::cast_from(3.0_f64) * t1459 * t30194;
    (t105818, t105822, t105826, t105830, t105834)
}
