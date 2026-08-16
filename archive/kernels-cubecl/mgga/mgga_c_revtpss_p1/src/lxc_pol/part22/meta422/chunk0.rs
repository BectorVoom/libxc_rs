//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2032/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2032<F: Float>(t2470: F, t4480: F, t2465: F, t11008: F, t1579: F, t2771: F, t1558: F, t836: F) -> (F, F, F, F) {
    let t14485 = t4480 * t2470;
    let t14486 = t2465 * t14485;
    let t14489 = t11008 * t1579 * t2771;
    let t14494 = t1558 * t836;
    (t14485, t14486, t14489, t14494)
}
