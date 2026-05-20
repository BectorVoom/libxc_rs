//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1795/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1795<F: Float>(t10192: F, t10194: F, t10415: F, t10416: F, t1310: F, t1315: F, t13207: F, t13435: F, t2320: F, t2328: F, t2372: F, t3813: F, t3821: F, t4151: F, t46126: F, t46129: F, t46137: F, t46233: F, t46349: F, t47632: F, t47634: F, t47648: F, t47662: F, t47676: F, t47681: F, t47687: F, t508: F, t511: F, t649: F, t651: F, t671: F, t94: F) -> F {
    let t47692 = -F::new(8.0) * t46126 * t671 - F::new(6.0) * t94 * t46137 * t508 - F::new(4.0) * t649 * t13207 - F::new(2.0) * t651 * t508 * t46233 - F::new(24.0) * t13435 * t2372 - F::new(12.0) * t10416 * t2372 + F::new(6.0) * t3821 * t4151 - F::new(12.0) * t2328 * t3813 - F::new(12.0) * t46129 * t508 - F::new(24.0) * t10194 * t1310 - F::new(4.0) * t10415 * t1310 - F::new(6.0) * t2320 * t3813 + F::new(4.0) * t1315 * t10192 + t511 * (t46349 + t47632 + t47634 + t47648 + t47662 + t47676 + t47681 + t47687);
    t47692
}
