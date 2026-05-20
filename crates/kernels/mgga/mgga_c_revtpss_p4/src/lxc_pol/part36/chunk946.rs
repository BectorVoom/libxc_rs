//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 946/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk946<F: Float>(t4279: F, t5911: F, t22604: F, t108: F, t105: F, t109: F, t1507: F, t1510: F, t22597: F, t22600: F, t22605: F, t22608: F, t22618: F, t5902: F, t5908: F, t5912: F, t97: F) -> F {
    let t22621 = t4279 * t5911;
    let t22624 = -t22604;
    let t22625 = t108 * t22624;
    let t22628 = -F::new(10.0) / F::new(27.0) * t97 * t22597 + F::new(10.0) / F::new(3.0) * t97 * t22600 + F::new(5.0) / F::new(3.0) * t97 * t22605 - F::new(440.0) / F::new(27.0) * t22608 * t109 + F::new(200.0) / F::new(9.0) * t5902 * t1510 - F::new(50.0) / F::new(9.0) * t1507 * t5908 - F::new(25.0) / F::new(3.0) * t1507 * t5912 - F::new(10.0) / F::new(27.0) * t105 * t22618 + F::new(10.0) / F::new(3.0) * t105 * t22621 + F::new(5.0) / F::new(3.0) * t105 * t22625;
    t22628
}
