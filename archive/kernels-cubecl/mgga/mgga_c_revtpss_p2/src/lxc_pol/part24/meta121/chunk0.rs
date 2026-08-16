//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 658/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk658<F: Float>(t4398: F, t762: F, t162: F, t2611: F, t227: F, t73: F, t1544: F, t853: F, t1559: F, t221: F, t2485: F, t2484: F) -> (F, F, F, F, F, F) {
    let t4399 = t4398 * t762;
    let t4401 = t2611 * t162;
    let t4415 = t227 * t73;
    let t4416 = t853 * t1544;
    let t4430 = t2485 * t221 * t1559;
    let t4431 = t2484 * t4430;
    (t4399, t4401, t4415, t4416, t4430, t4431)
}
