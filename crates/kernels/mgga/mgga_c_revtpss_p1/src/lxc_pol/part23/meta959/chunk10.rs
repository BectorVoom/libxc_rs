//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3231/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3231<F: Float>(t1298: F, t1832: F, t21639: F, t24501: F, t44126: F, t5023: F, t5501: F, t73273: F, t82060: F, t82400: F, t82402: F, t82404: F, t82406: F, t82410: F, t82415: F, t82418: F) -> F {
    let t85010 = -F::new(6.0) * t1298 * t24501 * t44126 * t5023 - F::new(3.0) * t1832 * t5023 * t73273 + F::new(6.0) * t21639 * t5023 * t5501 + t82060 - t82400 - t82402 - t82404 - t82406 - t82410 - t82415 + t82418;
    t85010
}
