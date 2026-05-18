//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1069/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1069<F: Float>(t7732: F, t8749: F, t2007: F, t2127: F, t33575: F, t33578: F, t33580: F, t33583: F, t33587: F, t33589: F, t33592: F, t33595: F, t33599: F, t34377: F, t7883: F, t8152: F) -> F {
    let t34379 = t7732 * t8749;
    let t34381 = -t2007 * t8152 - t2127 * t7883 - F::new(2.0) * t33575 - t33578 - t33580 - t33583 - F::new(2.0) * t33587 - F::new(2.0) * t33589 - F::new(2.0) * t33592 - t33595 - t33599 - F::new(2.0) * t34377 - F::new(2.0) * t34379;
    t34381
}
