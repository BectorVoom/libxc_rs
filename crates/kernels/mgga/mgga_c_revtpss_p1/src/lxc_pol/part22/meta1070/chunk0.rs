//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3827/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3827<F: Float>(t73552: F, t73576: F, t22404: F, t3920: F, t1445: F, t22445: F, t689: F, t13725: F, t1904: F, t2439: F, t1364: F, t22441: F, t786: F) -> (F, F, F, F, F) {
    let t73578 = t73552 / F::new(2.0) + t73576 / F::new(2.0);
    let t73587 = t22404 * t3920;
    let t73590 = t689 * t22445 * t1445;
    let t73593 = t2439 * t13725 * t1904;
    let t73598 = t786 * t22441 * t1364;
    (t73578, t73587, t73590, t73593, t73598)
}
