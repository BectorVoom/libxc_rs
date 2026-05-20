//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1183/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1183<F: Float>(t14652: F, t775: F, t2430: F, t4416: F, t14468: F, t832: F, t14633: F, t14643: F, t14649: F, t1553: F, t1555: F, t227: F, t229: F, t2634: F, t2639: F, t2642: F, t4409: F, t4415: F, t4417: F, t4420: F, t830: F, t833: F) -> F {
    let t14653 = t14652 * t775;
    let t14656 = t4416 * t2430;
    let t14659 = t832 * t14468;
    let t14662 = -t14633 * t229 - F::new(24.0) * t14643 * t4417 + F::new(60.0) * t14649 * t4415 - F::new(24.0) * t14653 * t4415 - F::new(12.0) * t14656 * t4415 + F::new(3.0) * t14659 * t227 - F::new(12.0) * t1553 * t2639 + F::new(3.0) * t1553 * t2642 + F::new(3.0) * t1555 * t2634 + F::new(6.0) * t4409 * t833 + F::new(6.0) * t4420 * t830;
    t14662
}
