//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 650/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk650<F: Float>(t1466: F, t602: F, t1469: F, t2275: F, t2282: F, t2299: F, t2306: F, t116: F, t1501: F) -> (F, F, F, F, F, F) {
    let t4173 = t1466 * t602;
    let t4201 = t2275 * t1469;
    let t4210 = t2282 * t1469;
    let t4227 = t2299 * t1469;
    let t4232 = t2306 * t1469;
    let t4248 = t1501 * t116;
    (t4173, t4201, t4210, t4227, t4232, t4248)
}
