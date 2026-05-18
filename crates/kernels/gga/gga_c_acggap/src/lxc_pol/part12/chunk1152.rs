//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1152/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1152<F: Float>(t11883: F, t642: F, t10761: F, t1679: F, t560: F, t32262: F, t495: F, t694: F, t9455: F, t9449: F, t96: F, t1674: F, t9108: F) -> (F, F, F, F, F, F) {
    let t36729 = t642 * t11883;
    let t36744 = F::new(2.0) * t1679 * t10761 * t560;
    let t36747 = F::new(6.0) * t694 * t32262 * t495;
    let t36750 = F::new(6.0) * t694 * t9455;
    let t36753 = F::new(2.0) * t96 * t9449;
    let t36755 = F::new(12.0) * t1674 * t9108;
    (t36729, t36744, t36747, t36750, t36753, t36755)
}
