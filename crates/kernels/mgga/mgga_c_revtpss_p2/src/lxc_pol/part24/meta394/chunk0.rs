//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1308/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1308<F: Float>(t258: F, t39552: F, t2454: F, t2455: F, t39494: F, t14545: F, t251: F, t786: F, t2710: F, t2793: F, t211: F, t9644: F) -> (F, F, F, F, F) {
    let t39554 = F::cast_from(0.88356352675825229576e-3_f64) * t39552 * t258;
    let t39557 = F::cast_from(0.20561456923286030469e-1_f64) * t2454 * t2455 * t39494;
    let t39597 = t14545 * t251;
    let t39598 = t786 * t39597;
    let t39633 = F::cast_from(0.20561456923286030469e-1_f64) * t2710 * t2793 * t39494;
    let t39643 = F::new(1.0) / t9644 / t211;
    (t39554, t39557, t39598, t39633, t39643)
}
